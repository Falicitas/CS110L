use crate::dwarf_data::{DwarfData, Error as DwarfError};
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
        // println!("stdin:{:?}", child.stdin);
        // println!("stdout:{:?}", child.stdout);
        let mut inferior = Inferior { child: child };
        // !When a process that has PTRACE_TRACEME enabled（指的就是unsafe块所执行的pre_exec(traceme)）
        // !calls exec（exec指的就是continue_run中的ptrace::cont。所以这两行隐含着 enabled PTRACE_TRACEME 的
        // !command 线程 pid 在 spawn 后会在程序刚开始运行时就处在暂停状态，直至执行 exec(pid)）,
        // !the operating system will load the specified program into the process,
        // !and then (before the new program starts running) it will pause the process using SIGTRAP.
        // !You should call waitpid on the child process to verify that it stops with signal SIGTRAP,
        // !in order to verify that everything is in working order (if this check fails, simply return None).
        // !You are welcome to call waitpid directly,
        // !or to use the Inferior::wait method that we have provided.
        //
        Some(inferior)
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

    /// kill the inferior, assume that the inferior is still alive
    pub fn kill(&mut self) {
        match self.child.kill() {
            // .unwrap_or_else(|_| println!("command wasn't running"));
            // .unwrap_or_else(|_| ());
            Ok(()) => println!("Killing running inferior (pid {})", self.child.id()), //? 暂时没确认为什么可以正常调用.id，在 kill 之后
            Err(_) => (),
        } // whatever it's killed, the message upon isn't needed.
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

    /// wake up the paused inferior process
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

    pub fn print_backtrace(&mut self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        let mut rip = regs.rip as usize;
        let mut rbp = regs.rbp as usize;
        loop {
            let _line = debug_data.get_line_from_addr(rip);
            let _func = debug_data.get_function_from_addr(rip);
            match (&_line, &_func) {
                (None, None) => println!("unknown func (source file not found)"),
                (Some(line), None) => println!("unknown func ({})", line),
                (None, Some(func)) => println!("{} (source file not found)", func),
                (Some(line), Some(func)) => println!("{} ({})", func, line),
            }
            if let Some(func) = _func {
                if func == "main" {
                    break;
                }
            } else {
                break;
            }
            rip = ptrace::read(self.pid(), (rbp + 8) as ptrace::AddressType)? as usize;
            rbp = ptrace::read(self.pid(), rbp as ptrace::AddressType)? as usize;
            // function's return address in running is stored 8 bytes above the saved %rbp value.
            // This return address is effectively %rip (the instruction pointer) for the previous stack frame.
        }
        Ok(())
    }
}
