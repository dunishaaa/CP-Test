mod tester;
use colored::*;
use std::env;
use tester::{
    my_file::{Code, File, FileType},
    Test,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = Code::new(&args[1]);
    let input = File::new(FileType::INPUT, &args[2]);
    let test: Test = Test::new(code, input, File::empty_expected(), File::empty_output());
    if test.compile() {
        test.run_code();
    } else {
        println!("{}", "ERROR: Failed to compile".color("red"));
    }
    //hola
}
