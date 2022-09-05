use std::process::exit;

use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::Status;
use crate::inferior::{self, Inferior};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: Vec<usize>,
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData

        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };
        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(), // 这里的target是cargo run X 的 X，
            //比如 samples/sleepy_print。传给Inferior的也是该target，而 r/run 是 args 的一部分
            history_path,
            readline,
            inferior: None,
            debug_data: debug_data,
            breakpoints: Vec::new(),
        } //self.target
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                //至此在(deet)已获取r 3
                DebuggerCommand::Run(args) => {
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                        self.inferior = None;
                    }
                    if let Some(inferior) = Inferior::new(&self.target, &args, &self.breakpoints) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // TODO (milestone 1): make the inferior run
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        match self.inferior.as_mut().unwrap().continue_run(None).unwrap() {
                            Status::Exited(exit_code) => {
                                println!("Child exited (status {})", exit_code);
                                self.inferior = None;
                            }
                            Status::Signaled(signal) => {
                                println!("Child exited due to signal {}", signal);
                                self.inferior = None;
                            }
                            Status::Stopped(signal, rip) => {
                                ////! RE 会触发这个
                                println!("Child stopped (signal {})", signal);
                                let _line = self.debug_data.get_line_from_addr(rip);
                                let _func = self.debug_data.get_function_from_addr(rip);
                                if _line.is_some() && _func.is_some() {
                                    println!("Stopped at {} ({})", _func.unwrap(), _line.unwrap());
                                }
                            }
                        }
                        // match
                    } else {
                        println!("Error starting subprocess");
                    }
                }

                DebuggerCommand::Continue => {
                    if let None = &mut self.inferior {
                        println!("Continue: Inferior doesn't exist");
                        continue;
                    }
                    match self.inferior.as_mut().unwrap().continue_run(None).unwrap() {
                        Status::Exited(exit_code) => {
                            println!("Child exited (status {})", exit_code);
                            self.inferior = None;
                        }
                        Status::Signaled(signal) => {
                            println!("Child exited due to signal {}", signal);
                            self.inferior = None;
                        }
                        Status::Stopped(signal, rip) => {
                            println!("Child stopped (signal {})", signal);
                            let _line = self.debug_data.get_line_from_addr(rip);
                            let _func = self.debug_data.get_function_from_addr(rip);
                            if _line.is_some() && _func.is_some() {
                                println!("Stopped at {} ({})", _func.unwrap(), _line.unwrap());
                            }
                        }
                    }
                } // println!("Inferior doesn't exist");

                DebuggerCommand::Quit => {
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                        self.inferior = None;
                    }
                    return;
                }

                DebuggerCommand::Backtrace => {
                    if self.inferior.is_none() {
                        println!(
                            "Error: you can not use backtrace when there is no process running"
                        );
                        continue;
                    }
                    // let addr = self.debug_data.get_line_from_addr();
                    if let Err(error) = self
                        .inferior
                        .as_mut()
                        .unwrap()
                        .print_backtrace(&self.debug_data)
                    {
                        println!("{}", error);
                    }
                }

                DebuggerCommand::Break(location) => {
                    if !location.starts_with("*") {
                        println!("Usage: b|break|breakpoint *address");
                        continue;
                    }
                    if let Some(addr) = parse_address(&location[1..]) {
                        if self.inferior.is_some() {
                            if let Err(_) = self.inferior.as_mut().unwrap().write_byte(addr, 0xcc) {
                                println!(" Invalid breakpoint address {:#x}", addr);
                            } else {
                                println!(
                                    "Set breakpoint {} at {}",
                                    self.breakpoints.len(),
                                    &location[1..]
                                );
                            }
                            //不中断程序，并在断点位置添加 INT = 0xcc
                        } else {
                            println!(
                                "Set breakpoint {} at {}",
                                self.breakpoints.len(),
                                &location[1..]
                            );
                            self.breakpoints.push(addr);
                        }
                    } else {
                        println!("Unrecognized address.");
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str()); //? 暂时没研究
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    } //? 暂时没研究
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                        //这里的cmd是 DebuggerCommand，判别了指令（enum类型）的类别后，将参数传给指令的值
                        //Millstone1 示例代码的 r 3，对应cmd = DebuggerCommand::Run(Vec<String>{3})
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
