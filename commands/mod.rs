pub mod cat;
pub mod cd;
pub mod cp;
pub mod help;

pub use cat::*;
pub use cd::*;
pub use cp::*;
pub use help::*;

use std::io::{ self, Write };
use std::env;

// Command: Echo
pub fn echo(args: &[&str]) -> io::Result<()> {
    println!("{}", args.join(" "));
    io::stdout().flush()?;
    Ok(())
}

// Command: Print Working Directory
pub fn pwd(_args: &[&str]) -> io::Result<()> {
    println!("{}", env::current_dir()?.display());
    io::stdout().flush()?;
    Ok(())
}

// Command: Clear
pub fn clear(_args: &[&str]) -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape code
    io::stdout().flush()?;
    Ok(())
}
