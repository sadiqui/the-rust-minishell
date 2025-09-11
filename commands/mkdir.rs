use std::path::Path;
use std::io::{ self };
use std::fs;


pub fn mkdir(args: &[&str]) -> io::Result<()> {
    for a in args {
        let path = Path::new(a);
        if path.exists() {
            eprintln!("mkdir: cannot create directory {:?}: File exists", path);
        } else {
            fs::create_dir(path)?;
        }
    }
    Ok(())
}
