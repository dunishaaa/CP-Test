use std::{fs, task::Context};
#[derive(Debug)]
#[allow(dead_code)]
enum FileType {
    INPUT,
    EXPECTED,
    CODE,
    OUTPUT,
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    file_type: FileType,
    content: String,
}
impl File {
    fn new(file_type: FileType, file: &str) -> Self {
        File {
            file_type,
            content: fs::read_to_string(file).expect("Not found!!!"),
        }
    }
}

fn main() {
    let input_values = File::new(FileType::INPUT, "../tests/a.in");
    let expected_output = File::new(FileType::EXPECTED, "../tests/expected");
    println!("{:}", &input_values.content);
    println!("{:}", &expected_output.content);
}
