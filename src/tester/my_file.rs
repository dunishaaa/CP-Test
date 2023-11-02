use std::fs;
#[derive(Debug)]
#[allow(dead_code)]
pub enum FileType {
    INPUT,
    EXPECTED,
    OUTPUT,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct File {
    pub file_type: FileType,
    pub path: String,
    pub content: String,
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct Code {
    pub path: String,
}
impl Code {
    pub fn new(path: &str) -> Self {
        Code {
            path: path.to_string(),
        }
    }
}
impl File {
    pub fn new(file_type: FileType, path: &str) -> Self {
        let error_message = format!("ERROR: File {} not found", path);
        File {
            file_type,
            path: path.to_string(),
            content: fs::read_to_string(path).expect(&error_message),
        }
    }
    pub fn empty_output() -> Self {
        File {
            file_type: FileType::OUTPUT,
            path: String::new(),
            content: String::new(),
        }
    }
    pub fn empty_expected() -> Self {
        File {
            file_type: FileType::EXPECTED,
            path: String::new(),
            content: String::new(),
        }
    }
}
