mod tester;
use std::env;
use tester::{my_file::File, my_file::FileType, Test};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = File::new(FileType::INPUT, "src/tests/a.in");
    let mut code = File::new(FileType::CODE, "src/tests/cowSign.cpp");
    let expected = File::new(FileType::EXPECTED, "src/tests/expected");

    if args.len() > 1 {
        println!("{:?}", &args[1..]);
        code = File::new(FileType::CODE, &args[1]);
        input = File::new(FileType::INPUT, &args[2]);
    }
    let mut test: Test = Test::new(input, code, expected, File::empty_output());
    test.compile();
    test.run_code();
    test.output = File::new(FileType::OUTPUT, "src/tests/a.out");
    if test.run_test() {
        println!("Passed!");
    } else {
        println!("Failed:((");
    }
}
