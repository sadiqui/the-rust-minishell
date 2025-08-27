use std::io::{ self };
use std::env;
use std::fs;
use colored::Colorize;
use std::path::Path;
// use dirs;

// fs::exists(path)
pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut res = Vec::new();
    let mut paths: Vec<String> = vec![];
    let mut flg = Flags::new();
    parse(args, &mut flg, &mut paths)?;
    
    
    // put this block of logic in a func
        let current_dir = env::current_dir()?;
        match fs::read_dir(current_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            dbg!(&entry);
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
        
    } // ends here

    

    Ok(())
}

struct Flags {
    a: bool,
    t: bool,
    F: bool,
    l: bool,
}

impl Flags {
    fn new() -> Self {
        Self { a: false, t: false, F: false, l: false }
    }
}

fn parse(args: &[&str], flg: &mut Flags, paths: &mut Vec<String>) -> io::Result<()> {
    for arg in args {
        if arg.starts_with('-') {
            let arg_flags = arg.trim_ascii_start();
            for f in arg_flags.chars() {
                if f == 't' {
                    flg.t = true;
                } else if f == 'F' {
                    flg.F = true;
                } else if f == 'l' {
                    flg.a = true;
                } else if f == 'a' {
                    flg.l = true;
                } else {
                    eprintln!("error invalid flag {}", f); // i'll write something better later
                }
            }
        } else {
            match fs::metadata(arg) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        paths.push(arg.to_owned().to_string());
                    } else {
                        eprintln!("{} is not a dir", arg); 
                    }
                },
                Err(_) => eprintln!("error invalid path" ), // Path does not exist or an error occurred accessing metadata
            }
        }
    }

    Ok(())
}


// if starts with "-" => flag
// else check if path is valid
