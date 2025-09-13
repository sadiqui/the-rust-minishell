use std::fs;
use std::io;
use std::path::Path;

/* todo:
Moving a directory into itself (should error)
Moving to a non-existent parent directory
Permission issues
*/

pub fn mv(args: &[&str]) -> io::Result<()> {
    if args.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "mv: missing or too many operands"));
    }
    let src = Path::new(args[0]);
    let dst = Path::new(args[1]);

    if !src.exists() {
        return Err(
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("mv: '{}' does not exist", src.display())
            )
        );
    }

    let final_dst = if dst.exists() && dst.is_dir() {
        dst.join(src.file_name().unwrap()) // move inside the directory
    } else {
        dst.to_path_buf() // move/rename to the target path directly
    };
    dbg!(&final_dst);

    if let Err(_) = fs::rename(src, &final_dst) {
        if src.is_file() {
            copy_file(src, &final_dst)?;
            fs::remove_file(src)?;
        } else if src.is_dir() {
            copy_dir(src, &final_dst)?;
            fs::remove_dir_all(src)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "mv: unsupported file type"));
        }
    }

    Ok(())
}

fn copy_file(src: &Path, dst: &Path) -> io::Result<u64> {
    let dst_file = if dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };
    fs::copy(src, dst_file)
}
fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    let dst_dir = if dst.exists() && dst.is_dir() {
        dst.join(src.file_name().unwrap())
    } else {
        dst.to_path_buf()
    };

    fs::create_dir_all(&dst_dir)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?; // recursive
        } else if src_path.is_file() {
            fs::copy(&src_path, &dst_path)?;
        } else {
            eprintln!("mv: skipping unsupported file type '{}'", src_path.display());
        }
    }

    Ok(())
}
