use std::io::{ self, Write };
use std::path::Path;
use std::fs;

// Command: Concatenate
pub fn cat(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        // Read from stdin
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        io::copy(&mut reader, &mut io::stdout())?;
    } else {
        // Read from existing files, one or more
        for &arg in args {
            let path = Path::new(arg);
            if path.is_dir() {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidInput, format!("'{}' is a directory", arg))
                );
            }

            let mut file = fs::File::open(path)?;
            io::copy(&mut file, &mut io::stdout())?;
        }
    }

    io::stdout().flush()?;
    Ok(())
}
