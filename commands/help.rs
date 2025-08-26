use std::io::{self};

// Command: Help (custom)
pub fn help(_args: &[&str]) -> io::Result<()> {
    println!("Available commands:");
    println!("  cd [dir]        - Change directory");
    println!("  pwd             - Show current directory");
    println!("  cat [file]      - Display file contents");
    println!("  cp [src] [dst]  - Copy file from src to dst");
    println!("  echo [text]     - Print text");
    println!("  clear           - Clear screen");
    println!("  help            - Show this help message");
    println!("  exit            - Exit shell");
    Ok(())
}
