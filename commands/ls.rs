use std::io::{ self };
use std::fs;
use colored::Colorize;
use std::path::Path;

pub fn ls(args: &[&str]) -> io::Result<()> {
    let mut paths = Vec::new();
    let mut flg = Flags::new();

    parse(args, &mut flg, &mut paths)?;

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for p in paths {
        let res = read_and_format_dir(Path::new(&p), &flg)?;
        for (i, f) in res.iter().enumerate() {
            if flg.l && i + 1 != res.len() {
                println!("{} ", f);
            } else {
                print!("{} ", f);
            }
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
                        if flg.l {
                            let metadata = entry.metadata()?;
                            let meta_str = format_metadata(&metadata);
                            let line = format!("{} {}", meta_str, styled);
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

/*
Fields:
File type + permissions (e.g. -rw-r--r--)
Number of hard links (e.g. 1)
Owner name (e.g. ahmed)
Group name (e.g. users)
File size (e.g. 1234)
Last modified date (e.g. Sep 7 00:12)
Filename (with symlink target if applicable)
*/

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use chrono::{ DateTime, Local };

/// Format file metadata similar to `ls -l`
/// Returns string with: permissions, links, uid, gid, size, time
pub fn format_metadata(metadata: &Metadata) -> String {
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };

    // Permissions (mode & 0o777)
    let mode = metadata.mode();
    let perms = mode_to_string(mode);

    // Number of hard links
    let nlink = metadata.nlink();

    // Owner / group (numeric for now)
    let uid = metadata.uid();
    let gid = metadata.gid();

    // File size
    let size = metadata.len();

    // Modification time
    let modified = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let time_str = datetime.format("%b %e %H:%M").to_string();

    format!("{}{} {:>3} {:>5} {:>5} {:>8} {}", file_type, perms, nlink, uid, gid, size, time_str)
}

/// Convert mode bits to `rwxr-xr-x` style
fn mode_to_string(mode: u32) -> String {
    let mut s = String::new();
    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];
    for (bit, ch) in flags {
        if (mode & bit) != 0 {
            s.push(ch);
        } else {
            s.push('-');
        }
    }
    s
}
