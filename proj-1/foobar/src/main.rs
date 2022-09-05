// use nix::sys::ptrace;
// use nix::sys::signal;
// use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
// use nix::unistd::Pid;
// use std::os::unix::process::CommandExt;
// use std::process::Child;
// use std::process::Command;
// fn child_traceme() -> Result<(), std::io::Error> {
//     ptrace::traceme().or(Err(std::io::Error::new(
//         std::io::ErrorKind::Other,
//         "ptrace TRACEME failed",
//     )))
// }

// pub fn pid(child: &mut Child) -> Pid {
//     nix::unistd::Pid::from_raw(child.id() as i32)
// }
use std;

pub struct c {
    t: String,
}

impl c {
    fn len(&mut self) {

        // let x = self.t;
        // let x = self.t;
        // let x = self;
        // let y = self;
        // println!("{}", self.t);
        // println!("{}", self.t);
    }
}

fn main() {
    // let mut cmd = Command::new("samples/count"); //打印1 2 3 4 5
    //                                              // cmd.args(args);
    //                                              // unsafe {
    //                                              //     cmd.pre_exec(child_traceme);
    //                                              // }
    // let mut child = cmd.spawn().ok().unwrap();
    // // ptrace::cont(pid(&mut child), None).unwrap();
    // // waitpid(pid(&mut child), None).unwrap();
    // let k = 10;
    // assert_eq!(Some(4).unwrap_or_else(|| None::<i32>), 4);
    // assert_eq!(, 4);
    // Some(4).unwrap_or_else(|| println!("hhhh"));
    let x = Vec<usize>::new();
}
