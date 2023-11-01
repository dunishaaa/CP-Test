mod tester;
use tester::my_file::{File, FileType};

/*
enum FilePath {
    CODE(String),
    EXPECTED(String),
    OUTPUT(String),
    INPUT(String),
}
*/

fn main() {
    /*
    let _code_path = FilePath::CODE("src/tests/cowSign.cpp".to_string());
    let _expected_path = FilePath::EXPECTED("src/tests/expected".to_string());
    let _output_path = FilePath::OUTPUT("src/tests/a.out".to_string());
    let _input_path = FilePath::INPUT("src/tests/a.in".to_string());
    */
    let input_values = File::new(FileType::INPUT, "src/tests/a.in");
    let code = File::new(FileType::CODE, "src/tests/cowSign.cpp");

    if code.compile() {
        println!("Compilation succesful. Now testing...");
        input_values.run_code();
        let expected_output = File::new(FileType::EXPECTED, "src/tests/expected");
        if expected_output.run_test() {
            println!("Passed");
        } else {
            println!("Failed");
        }
    } else {
        println!("Compilation unsuccesful:(((");
    }
}
