use std::{fs, io, path::PathBuf};

pub fn read_input(file_path: &str) -> Result<String, std::io::Error> {
    let exe_path = std::env::current_exe()?;
    let dir_name = exe_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "crash and burn zzz"))?
        .to_str()
        .unwrap();
    let input_path = PathBuf::from(dir_name).join(file_path);
    fs::read_to_string(input_path)
}


pub fn split_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}
