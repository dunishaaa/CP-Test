pub mod my_file;
use my_file::File;
pub struct FilesCompare {
    expected: File,
    actual_output: File,
}

impl FilesCompare {
    pub fn new(expected: File, actual_output: File) -> Self {
        FilesCompare {
            expected,
            actual_output,
        }
    }
    pub fn test(self: Self) -> bool {
        let expected_cos = self
            .expected
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();
        let actual_cos = self
            .actual_output
            .content
            .split(' ')
            .filter(|x| x != &"\n")
            .collect::<Vec<&str>>();

        println!("{:?}", expected_cos);
        println!("{:?}", actual_cos);
        self.expected.content == self.actual_output.content
    }
}
