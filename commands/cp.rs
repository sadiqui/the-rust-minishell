use std::{ fs, io::{ self, Error, ErrorKind }, path::PathBuf };

// Command: Copy
pub fn cp(args: &[&str]) -> io::Result<()> {
    if args.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Usage: cp [src] [dst]"));
    }

    let src = PathBuf::from(args[0]);
    let mut dst = PathBuf::from(args[1]);

    // Check source file
    if !src.exists() {
        return Err(
            Error::new(ErrorKind::NotFound, format!("Source file '{}' not found", src.display()))
        );
    }
    if !src.is_file() {
        return Err(
            Error::new(ErrorKind::InvalidInput, format!("Source '{}' is not a file", src.display()))
        );
    }

    // If dst is a directory, append the source filename
    if dst.exists() && dst.is_dir() {
        dst = dst.join(
            src.file_name().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid source filename"))?
        );
    }

    fs::copy(&src, &dst)?; // Evalute to value or Err()
    Ok(())
}
