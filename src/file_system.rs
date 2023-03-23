use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn create_file(path: &str) -> io::Result<()> {
    File::create(path)?;
    Ok(())
}

pub fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &str, data: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())
}

pub fn append_file(path: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(data.as_bytes())
}

pub fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir(path)
}

pub fn delete_directory(path: &str) -> io::Result<()> {
    fs::remove_dir(path)
}

pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut names = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            names.push(name.to_string());
        }
    }

    Ok(names)
}

pub fn move_item(src: &str, dest: &str) -> io::Result<()> {
    fs::rename(src, dest)
}

pub fn copy_item(src: &str, dest: &str) -> io::Result<()> {
    if Path::new(src).is_file() {
        fs::copy(src, dest)?;
    } else {
        copy_directory(src, dest)?;
    }
    Ok(())
}

fn copy_directory(src: &str, dest: &str) -> io::Result<()> {
    fs::create_dir(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dest).join(path.file_name().unwrap());

        if path.is_dir() {
            copy_directory(path.to_str().unwrap(), dest_path.to_str().unwrap())?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }
    Ok(())
}
