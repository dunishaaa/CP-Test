use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
pub enum FileType {
    INPUT,
    EXPECTED,
    CODE,
    OUTPUT,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct File {
    pub file_type: FileType,
    pub content: String,
}
impl File {
    pub fn new(file_type: FileType, file: &str) -> Self {
        let error_message = format!("Not found {}", file);
        File {
            file_type,
            content: fs::read_to_string(file).expect(&error_message),
        }
    }
}
