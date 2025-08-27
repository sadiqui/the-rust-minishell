use std::io::{ self };
use std::env;
use std::fs;
use colored::Colorize;
// use dirs;

// fs::exists(path)
pub fn ls(args: &[&str]) -> io::Result<()> {
    // if no args ls current dir
    if args.is_empty() {
        let current_dir = env::current_dir()?;
        match fs::read_dir(current_dir) {
            Ok(entries) => {
                let mut res = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let file_name = entry.file_name();
                            if !file_name.to_string_lossy().starts_with('.') {
                                if entry.file_type()?.is_dir() {
                                    res.push(file_name.to_string_lossy().blue().bold());
                                } else if entry.file_type()?.is_file() {
                                    res.push(file_name.to_string_lossy().white());
                                } else {
                                    res.push(file_name.to_string_lossy().green().bold());
                                }
                            }
                        }
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                // print here
                for f in res {
                    print!("{} ", f);
                }
                println!();
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    }
    Ok(())
}

// if starts with "-" => flag
// else check if path is valid
