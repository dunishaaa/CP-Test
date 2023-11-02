pub mod my_file;
use colored::*;
use my_file::{Code, File, FileType};
use std::{
    fs,
    process::{Command, Stdio},
    str,
};

#[derive(Debug)]
pub struct Test {
    pub code: Code,
    pub input: File,
    pub expected: File,
    pub output: File,
    partial_path: String,
}
impl Test {
    fn parse_partial_path(path: &str) -> String {
        let mut index = (path.len() as i8) - 1;
        for ch in path.chars().rev() {
            if ch == '/' {
                break;
            }
            index = index - 1;
        }
        if index <= 0 {
            return "".to_string();
        } else {
            let ind = index as usize;
            return path[0..ind + 1].to_string();
        }
    }
    pub fn new(code: Code, input: File, expected: File, output: File) -> Self {
        let path_to_parse = code.path.to_string();
        Test {
            code,
            input,
            expected,
            output,
            partial_path: Test::parse_partial_path(&path_to_parse),
        }
    }

    pub fn compile(self: &Self) -> bool {
        println!(
            "{} {}",
            "Compiling".color("blue"),
            self.code.path.color("blue")
        );
        let file_name = format!("{}main", self.partial_path);
        let compilation_status = Command::new("g++")
            .arg(&self.code.path)
            .arg("-o")
            .arg(file_name)
            .status()
            .expect("ERROR: Could not compile file !!!");
        if compilation_status.success() {
            println!("{}", "Compile successful!".color("green"));
        } else {
            println!("{}", compilation_status);
        }
        return compilation_status.success();
    }

    pub fn run_code(self: &Self) {
        if let FileType::INPUT = self.input.file_type {
            println!("{}", "Running code...".color("purple"));
            let input = Command::new("echo")
                .arg(&self.input.content)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to create process");
            let echo_out = input.stdout.expect("Failed to open process");
            let execution_path = format!("./{}main", self.partial_path);
            let save_out_path = format!("{}a.out", self.partial_path);

            let test = Command::new(execution_path)
                .stdin(Stdio::from(echo_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: execution error");
            let ans = test
                .wait_with_output()
                .expect("Failed to retreive answer output")
                .stdout;
            let code_stdout = str::from_utf8(&ans).unwrap();
            fs::write(save_out_path, code_stdout).expect("Could not write to file!");
            println!("{}", "----------------------".color("purple"));
            println!("OUTPUT:\n\n{}", code_stdout);
        }
    }
    #[allow(dead_code)]
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
