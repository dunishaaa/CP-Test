use std::{
    fs,
    process::{self, Command, Stdio},
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
    let input_values = File::new(FileType::INPUT, "../tests/a.in");
    let expected_output = File::new(FileType::EXPECTED, "../tests/expected");
    println!("{:}", &input_values.content);
    //    println!("{:}", &expected_output.content);
    let status = Command::new("g++")
        .arg("../tests/cowSign.cpp")
        .arg("-o")
        .arg("../tests/main")
        .status()
        .expect("Valio burguer");

    if status.success() {
        println!("Compilation succesful. Now testing...");
        let input = Command::new("echo")
            .arg("1 9")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to create process");
        let echo_out = input.stdout.expect("Failed to open process");

        let test = Command::new("./../tests/main")
            .stdin(Stdio::from(echo_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Hijole");
        let ans = test.wait_with_output().expect("kk");
        //        let idk = ans.stdout;
        println!("{:?}", ans);
    //        assert_eq!("1 2 3 4 5 6 7 8 \n", ans.stdout);
    } else {
        println!("Compilation unsuccesful:(((");
    }
    //    compile.spawn();
}
