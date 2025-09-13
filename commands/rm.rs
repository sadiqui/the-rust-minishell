use std::fs;
use std::io;
use std::path::Path;

pub fn rm(args: &[&str]) -> io::Result<()> {
    // check if file exists
    // check if its file or directory
    // check if the flag -r is set

    for a in args {
        if *a == "-r" {
            continue;
        }
        let path = Path::new(a);
        if !path.exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("rm: '{}' does not exist", path.display())
                )
            );
        }
        if path.is_file() {
            fs::remove_file(path)?;
        }
        if path.is_dir() {
            if check_flag(args) {
                fs::remove_dir_all(path)?;
            } else {
                return Err(
                    io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        format!("rm: '{}' is a directory", path.display())
                    )
                );
            }
        }
    }

    Ok(())
}

fn check_flag(args: &[&str]) -> bool {
    for a in args {
        if *a == "-r" {
            return true;
        }
    }
    false
}
