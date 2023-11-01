pub mod my_file;
use my_file::{File, FileType};
use std::{
    fs,
    process::{Command, Stdio},
    str,
};

pub struct Test {
    pub code: File,
    pub input: File,
    pub expected: File,
    pub output: File,
}
impl Test {
    pub fn new(code: File, input: File, expected: File, output: File) -> Self {
        Test {
            code,
            input,
            expected,
            output,
        }
    }
    pub fn compile(self: &Self) -> bool {
        println!("Compiling...");
        if let FileType::CODE = self.code.file_type {
            let compilation_status = Command::new("g++")
                .arg(&self.code.path)
                .arg("-o")
                .arg("src/tests/main")
                .status()
                .expect("ERROR: Could not compile file !!!");

            println!("Compiling finished...");
            return compilation_status.success();
        } else {
            println!("Incorrect filetype");
            false
        }
    }

    pub fn run_code(self: &Self) {
        if let FileType::INPUT = self.input.file_type {
            println!("Running code!");
            let input = Command::new("echo")
                .arg(&self.input.content)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to create process");
            let echo_out = input.stdout.expect("Failed to open process");

            let test = Command::new("./src/tests/main")
                .stdin(Stdio::from(echo_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: execution error");
            let ans = test
                .wait_with_output()
                .expect("Failed to retreive answer output")
                .stdout;
            let code_stdout = str::from_utf8(&ans).unwrap();
            fs::write("src/tests/a.out", code_stdout).expect("Could not write to file!");
        }
    }
    pub fn run_test(self: Self) -> bool {
        println!("Testing...");
        let output_values = File::new(FileType::OUTPUT, "src/tests/a.out");
        let expected_cos = &self
            .expected
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();
        let actual_cos = &output_values
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();

        println!("Expected: {:?}", expected_cos);
        println!("Actual:   {:?}", actual_cos);
        expected_cos == actual_cos
    }
}
