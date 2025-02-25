use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::Inferior;
use crate::inferior::Status;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, u8>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // #begin: Initializes DwarfData object
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
        // #end

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        // #begin: return Instantiated Object
        Debugger {
            target: target.to_string(),
            // target here is the X of "cargo run X",
            // like "samples/sleepy_print".
            history_path,
            readline,
            inferior: None,
            debug_data: debug_data,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                // (deet) captured r 3, do a match, and let args = ["3"]
                DebuggerCommand::Run(args) => {
                    // #begin: kill the inferior if it exists
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                        self.inferior = None;
                    }
                    // #end

                    // #begin: try to instantiate a inferior
                    if let Some(inferior) =
                        Inferior::new(&self.target, &args, &mut self.breakpoints)
                    {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // #begin: run inferior until inferior's state occurs to change.
                        // use self.inferior.as_mut().unwrap() to get a mutable reference to the Inferior object
                        match self
                            .inferior
                            .as_mut()
                            .unwrap()
                            .continue_run(None, &mut self.breakpoints)
                            .unwrap()
                        {
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
                        // #end
                    } else {
                        println!("Error starting subprocess");
                    }
                    // #end
                }

                DebuggerCommand::Continue => {
                    if let None = &mut self.inferior {
                        println!("Error: Inferior doesn't exist");
                        continue;
                    }
                    // #begin: run inferior until inferior's state occurs to change.
                    match self
                        .inferior
                        .as_mut()
                        .unwrap()
                        .continue_run(None, &mut self.breakpoints)
                        .unwrap()
                    {
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
                    // #end
                }

                DebuggerCommand::Quit => {
                    // #begin: kill the inferior if it exists
                    if self.inferior.is_some() {
                        self.inferior.as_mut().unwrap().kill();
                        self.inferior = None;
                    }
                    // #end
                    return;
                }

                DebuggerCommand::Backtrace => {
                    if self.inferior.is_none() {
                        println!(
                            "Error: you can not use backtrace when there is no process running"
                        );
                        continue;
                    }

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
                    let breakpoint_addr;
                    if location.starts_with("*") {
                        if let Some(address) = parse_address(&location[1..]) {
                            breakpoint_addr = address;
                        } else {
                            println!("Invalid address");
                            continue;
                        }
                    } else if let Some(line) = usize::from_str_radix(&location, 10).ok() {
                        //? str_radixq
                        println!("{}", line);
                        if let Some(address) = self
                            .debug_data
                            .get_addr_for_line(Some(&format!("{}.c", &self.target)), line)
                        {
                            breakpoint_addr = address;
                        } else {
                            println!("Invalid line number");
                            continue;
                        }
                    } else if let Some(address) = self
                        .debug_data
                        .get_addr_for_function(Some(&format!("{}.c", &self.target)), &location)
                    {
                        breakpoint_addr = address;
                    } else {
                        println!("Usage: b|break|breakpoint *address|line|func");
                        continue;
                    }

                    if self.inferior.is_some() {
                        if let Some(instruction) = self
                            .inferior
                            .as_mut()
                            .unwrap()
                            .write_byte(breakpoint_addr, 0xcc)
                            .ok()
                        {
                            println!(
                                "Set breakpoint {} at {:#x}",
                                self.breakpoints.len(),
                                breakpoint_addr
                            );
                            self.breakpoints.insert(breakpoint_addr, instruction);
                        } else {
                            println!("Invalid breakpoint address {:#x}", breakpoint_addr);
                        }
                    } else {
                        // when the inferior is initiated, these breakpoints will be installed
                        println!(
                            "Set breakpoint {} at {:#x}",
                            self.breakpoints.len(),
                            breakpoint_addr
                        );
                        self.breakpoints.insert(breakpoint_addr, 0);
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(qeet) ") {
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
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                        // cmd here is DebuggerCommand, it will deliver the instruction's value after parse the type of instruction
                        // Example code shown in Millstone1 "r 3" corresponds to cmd = DebuggerCommand::Run(Vec<String>{3})
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
