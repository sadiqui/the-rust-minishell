use std::io::{ self, Error, ErrorKind };
use std::env;
use dirs;

// Command: Change Directory
pub fn cd(args: &[&str]) -> io::Result<()> {
    // Determine target path
    let path = if args.is_empty() {
        dirs::home_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "Home directory not found"))?
    } else {
        std::path::PathBuf::from(args[0])
    };

    // Avoid transient “not found” errors caused by weird I/O timing
    if !path.exists() || !path.is_dir() {
        return Err(
            Error::new(
                ErrorKind::NotFound,
                format!("Directory '{}' does not exist", path.display())
            )
        );
    }

    // Change directory, if fail return Err()
    env::set_current_dir(path)?;
    Ok(())
}
