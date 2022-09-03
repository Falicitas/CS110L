use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::process::Child;
pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>) -> Option<Inferior> {
        // TODO: implement me!
        use std::os::unix::process::CommandExt;
        use std::process::Command;
        let mut cmd = Command::new(target);
        cmd.args(args);
        unsafe {
            cmd.pre_exec(child_traceme);
        }
        let child = cmd.spawn().ok()?;
        let mut inferior = Inferior { child: child };
        //  match inferior.continue_run(None).ok()? {
        //     Status::Exited(exit_code) => println!("Child exited (status {})", exit_code),
        //     Status::Signaled(signal) => println!("Child exited due to signal {}", signal),
        //     Status::Stopped(signal, rip) => println!("Child stopped by signal {} at address {:#x}", signal, rip),
        // }
        Some(inferior)
        ////! 看 bilibili Rust 补充知识，PKUFlyingPig 的历史 Commit 前后对比，一个个做 Millstone
        // println!(
        //     "Inferior::new not implemented! target={}, args={:?}",
        //     target, args
        // );
        // None
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }
    pub fn continue_run(&self, signal: Option<signal::Signal>) -> Result<(), nix::Error> {
        ptrace::cont(self.pid(), signal)?;
        // 简单的理解是，cont指令将 inferior 的线程再次启动，另外类似的指令是PTRACE_SYSCALL
        // 对 ptrace 的主观理解：
        // Traceme用于跟踪。
        // PTRACE_PEEKUSER 读取子进程运行时eax寄存器的值。要在子程序运行完，即 wait 后调用
        // 一篇入门 ptrace 笔记：https://omasko.github.io/2018/04/19/ptrace%E5%AD%A6%E4%B9%A0%E7%AC%94%E8%AE%B0I/
        Ok(match self.wait(None)? {
            Status::Exited(exit_code) => println!("Child exited (status {})", exit_code),
            Status::Signaled(signal) => println!("Child exited due to signal {}", signal),
            Status::Stopped(signal, rip) => {
                println!("Child stopped by signal {} at address {:#x}", signal, rip)
            }
        })
    }
}
