//! A simple mini-shell implementation in Rust

mod commands;
use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::io::{self, stdin, Error, ErrorKind, Write};

type CommandFn = fn(&[&str]) -> io::Result<()>;

struct MiniShell {
    command_map: HashMap<&'static str, CommandFn>,
    should_exit: bool,
}

impl MiniShell {
    fn new() -> Self {
        let mut command_map = HashMap::new();
        
        // Use explicit type casting to ensure consistent function pointer types
        command_map.insert("cd", commands::cd as CommandFn);
        command_map.insert("echo", commands::echo as CommandFn);
        command_map.insert("pwd", commands::pwd as CommandFn);
        command_map.insert("clear", commands::clear as CommandFn);
        command_map.insert("help", commands::help as CommandFn);
        command_map.insert("exit", (|_| Ok(())) as CommandFn);

        Self {
            command_map,
            should_exit: false,
        }
    }

    fn run(&mut self) -> io::Result<()> {
        while !self.should_exit {
            if let Err(e) = self.run_prompt() {
                match e.kind() {
                    io::ErrorKind::UnexpectedEof => break,
                    _ => eprintln!("{}", format!("Error: {}", e).red()),
                }
            }
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let current_dir = env::current_dir()?
            .display()
            .to_string()
            .cyan()
            .bold();
        
        print!("[{}] $ ", current_dir);
        io::stdout().flush()?;
        
        let mut user_input = String::new();
        if stdin().read_line(&mut user_input)? == 0 {
            println!("\nexit");
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }
        
        self.process_input(&user_input.trim())
    }

    fn process_input(&mut self, input: &str) -> io::Result<()> {
        if input.is_empty() {
            return Ok(());
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        match args[0] {
            "exit" => {
                self.should_exit = true;
                Ok(())
            },
            cmd => match self.command_map.get(cmd) {
                Some(&func) => func(&args[1..]),
                None => Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Command '{}' not found. Type 'help' for available commands.", cmd),
                )),
            }
        }
    }
}

fn main() -> io::Result<()> {
    MiniShell::new().run()
}
