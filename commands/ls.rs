use std::io::{ self };
use std::fs;
use colored::Colorize;
use std::path::Path;
// use dirs;

// fs::exists(path)
pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flg = Flags::new();

    parse(args, &mut flg, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for p in paths {
        let res = read_and_format_dir(Path::new(&p), &flg)?;
        for f in res {
            print!("{} ", f);
        }
        println!();
    }

    Ok(())
}

#[allow(non_snake_case)]
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
            let arg_flags = arg.trim_start_matches('-');
            // dbg!(&arg_flags);
            for f in arg_flags.chars() {
                if f == 't' {
                    flg.t = true;
                } else if f == 'F' {
                    flg.F = true;
                } else if f == 'l' {
                    flg.l = true;
                } else if f == 'a' {
                    flg.a = true;
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
                }
                Err(_) => eprintln!("error invalid path"), // Path does not exist or an error occurred accessing metadata
            }
        }
    }

    Ok(())
}

fn read_and_format_dir(path: &Path, flg: &Flags) -> io::Result<Vec<String>> {
    let mut res = Vec::new();
    // dbg!(&flg.l);
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        // here if -l

                        // respect `-a` flag
                        if !flg.a && file_name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                        let styled = if entry.file_type()?.is_dir() {
                            file_name.to_string_lossy().blue().bold().to_string()
                        } else if entry.file_type()?.is_file() {
                            file_name.to_string_lossy().white().to_string()
                        } else {
                            file_name.to_string_lossy().green().bold().to_string()
                        };
                        if flg.l { // todo: finish this shit
                            let metadata = entry.metadata()?;
                            let size = metadata.len();
                            let modified = metadata.modified()?;
                            let modified_time: chrono::DateTime<chrono::Local> = modified.into();

                            use std::os::unix::fs::PermissionsExt;
                            let perms = metadata.permissions();
                            let mode = perms.mode();

                            // format the "long listing" line
                            let line = format!(
                                "{:o} {:>8} {} {}",
                                mode & 0o777, // octal perms
                                size, // file size
                                modified_time.format("%Y-%m-%d %H:%M"), // timestamp
                                styled // colored filename
                            );
                            res.push(line);
                        } else {
                            res.push(styled);
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory {}: {}", path.display(), e),
    }

    Ok(res)
}
