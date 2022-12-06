use anyhow::Result;
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn read_file_to_string<T: AsRef<Path>>(name: T) -> Result<String> {
    let mut file = File::open(name)?;
    let mut res = String::new();
    file.read_to_string(&mut res)?;
    Ok(res)
}

pub fn write_string_to_file<T: AsRef<Path>, S: Display>(path: T, content: S) -> Result<()> {
    let mut f = File::create(path)?;
    write!(f, "{}", content)?;
    Ok(())
}
