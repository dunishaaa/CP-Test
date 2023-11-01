use std::{
    fs,
    process::{Command, Stdio},
    str,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum FileType {
    CODE,
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
impl File {
    pub fn new(file_type: FileType, path: &str) -> Self {
        let error_message = format!("ERROR: File {} not found", path);
        File {
            file_type,
            path: path.to_string(),
            content: fs::read_to_string(path).expect(&error_message),
        }
    }
    pub fn compile(self: Self) -> bool {
        if let FileType::CODE = self.file_type {
            let compilation_status = Command::new("g++")
                .arg(self.path)
                .arg("-o")
                .arg("src/tests/main")
                .status()
                .expect("ERROR: Could not compile file !!!");

            return compilation_status.success();
        } else {
            println!("Incorrect filetype");
            false
        }
    }
    pub fn run_code(self: Self) {
        if let FileType::INPUT = self.file_type {
            let input = Command::new("echo")
                .arg(self.content)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to create process");
            let echo_out = input.stdout.expect("Failed to open process");

            let test = Command::new("./src/tests/main")
                .stdin(Stdio::from(echo_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("Hijole");
            let ans = test
                .wait_with_output()
                .expect("Failed to retreive answer output")
                .stdout;
            let idk = str::from_utf8(&ans).unwrap();
            fs::write("src/tests/a.out", idk).expect("Could not write to file!");
        }
    }
    pub fn run_test(self: Self) -> bool {
        let output_values = File::new(FileType::OUTPUT, "src/tests/a.out");

        let expected_cos = &self
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();
        let actual_cos = &output_values
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();

        expected_cos == actual_cos
    }
}
