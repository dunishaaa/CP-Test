mod tester;
use std::{
    fs,
    process::{Command, Stdio},
    str,
};
use tester::{
    my_file::{File, FileType},
    FilesCompare,
};

fn main() {
    let input_values = File::new(FileType::INPUT, "src/tests/a.in");
    let compilation_status = Command::new("g++")
        .arg("src/tests/cowSign.cpp")
        .arg("-o")
        .arg("src/tests/main")
        .status()
        .expect("Valio burguer");

    if compilation_status.success() {
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
        let ans = test
            .wait_with_output()
            .expect("Failed to retreive answer output")
            .stdout;
        let idk = str::from_utf8(&ans).unwrap();
        fs::write("src/tests/a.out", idk).expect("Could not write to file!");
        let expected_output = File::new(FileType::EXPECTED, "src/tests/expected");
        let output_values = File::new(FileType::OUTPUT, "src/tests/a.out");
        let test = FilesCompare::new(expected_output, output_values);
        if test.test() {
            println!("Passed");
        } else {
            println!("Failed");
        }
    } else {
        println!("Compilation unsuccesful:(((");
    }
}
