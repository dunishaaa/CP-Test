use std::{
    fs,
    process::{Command, Stdio},
    str,
};
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
    let input_values = File::new(FileType::INPUT, "src/tests/a.in");
    let expected_output = File::new(FileType::EXPECTED, "src/tests/expected");
    println!("{:}", &input_values.content);
    //    println!("{:}", &expected_output.content);
    let status = Command::new("g++")
        .arg("src/tests/cowSign.cpp")
        .arg("-o")
        .arg("src/tests/main")
        .status()
        .expect("Valio burguer");

    if status.success() {
        println!("Compilation succesful. Now testing...");
        let input = Command::new("echo")
            .arg(input_values.content)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to create process");
        let echo_out = input.stdout.expect("Failed to open process");

        let test = Command::new("./src/tests/main")
            .stdin(Stdio::from(echo_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Hijole");
        let ans = test.wait_with_output().expect("kk").stdout;
        let idk = str::from_utf8(&ans).unwrap();
    } else {
        println!("Compilation unsuccesful:(((");
    }
}
