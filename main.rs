// Set up modules for color printing, collections,
// environment handling, i/o with error handling.
mod commands;
use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::io::{ self, stdin, Error, ErrorKind, Write };

// Store commands in a map and execute them via
// function pointer type (store address of funtcion)
type CommandFn = fn(&[&str]) -> io::Result<()>;
struct MiniShell {
    command_map: HashMap<&'static str, CommandFn>,
    should_exit: bool,
}

impl MiniShell {
    // Construct a command-to-function lookup table
    fn new() -> Self {
        let mut command_map = HashMap::new();

        // Explicit type casting for consistent function pointer types
        command_map.insert("cd", commands::cd as CommandFn);
        command_map.insert("echo", commands::echo as CommandFn);
        command_map.insert("pwd", commands::pwd as CommandFn);
        command_map.insert("clear", commands::clear as CommandFn);
        command_map.insert("help", commands::help as CommandFn);
        command_map.insert("exit", (|_| Ok(())) as CommandFn); // Inline closure

        Self { command_map, should_exit: false }
    }

    // Main loop of the shell
    // Run commands until exit or EOF is sent
    fn run(&mut self) -> io::Result<()> {
        while !self.should_exit {
            if let Err(e) = self.run_prompt() {
                match e.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break; // Break on (Ctrl+D / end of input)
                    }
                    _ => eprintln!("{}", format!("Error: {}", e).red()),
                }
            }
        }
        Ok(())
    }

    // Display prompt with current directory and process user input
    fn run_prompt(&mut self) -> io::Result<()> {
        let current_dir = env::current_dir()?.display().to_string().cyan().bold();

        print!("[{}] $ ", current_dir);
        io::stdout().flush()?; // Force immediate appear

        let mut user_input = String::new();
        if stdin().read_line(&mut user_input)? == 0 {
            println!("\nexit");
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        self.process_input(&user_input.trim())
    }

    // Look up the command in the command_map and execute it
    fn process_input(&mut self, input: &str) -> io::Result<()> {
        if input.is_empty() {
            return Ok(()); // No-op
        }

        // Ex: ["echo", "hello", "world"]
        let args: Vec<&str> = input.split_whitespace().collect();
        match args[0] {
            "exit" => {
                self.should_exit = true;
                Ok(())
            }

            cmd => // Ex: "echo"
                match self.command_map.get(cmd) {
                    Some(&func) => func(&args[1..]), // Ex, "hello", "world"
                    None =>
                        Err(
                            Error::new(
                                ErrorKind::NotFound,
                                format!("Command '{}' not found. Type 'help' for available commands.", cmd)
                            )
                        ),
                }
        }
    }
}

fn main() -> io::Result<()> {
    ctrlc // Handle Ctrl+C gracefully, no crash
        ::set_handler(|| {
            println!("\n(Use Ctrl+D or type 'exit' to quit)");
        })
        .expect("Error setting Ctrl-C handler");

    MiniShell::new().run()
}
