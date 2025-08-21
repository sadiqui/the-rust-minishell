// pub mod cat;
// pub mod cd;
// pub mod cp;
// pub mod echo;
// pub mod exit;
// pub mod ls;
// pub mod mkdir;
// pub mod mv;
// pub mod pwd;
// pub mod rm;

// pub use cat::*;
// pub use cd::*;
// pub use cp::*;
// pub use echo::*;
// pub use exit::*;
// pub use ls::*;
// pub use mkdir::*;
// pub use mv::*;
// pub use pwd::*;
// pub use rm::*;

use std::io::{ self, Error, ErrorKind };
use std::env;
use dirs;

// Command: Change Directory
pub fn cd(args: &[&str]) -> io::Result<()> {
    let path = if args.is_empty() {
        // Give home directory or Err() if not found
        dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "Home directory not found"))?
    } else {
        // Give target path from cd (first) argument
        std::path::PathBuf::from(args[0])
    };
    // Change directory, if fail return Err()
    env::set_current_dir(path)?;
    Ok(())
}

// Command: Echo
pub fn echo(args: &[&str]) -> io::Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}

// Command: Print Working Directory
pub fn pwd(_args: &[&str]) -> io::Result<()> {
    println!("{}", env::current_dir()?.display());
    Ok(())
}

// Command: Clear
pub fn clear(_args: &[&str]) -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape code
    Ok(())
}

// Command: Help (custom)
pub fn help(_args: &[&str]) -> io::Result<()> {
    println!("Available commands:");
    println!("  cd [dir]    - Change directory");
    println!("  echo [text] - Print text");
    println!("  pwd         - Show current directory");
    println!("  clear       - Clear screen");
    println!("  help        - Show this help");
    println!("  exit        - Exit shell");
    Ok(())
}
