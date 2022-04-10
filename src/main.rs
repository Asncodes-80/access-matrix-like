use std::fs;
fn main() {
    let file_content = file_picker("sample.txt");
    println!("{:?}", file_content[0][3]);
}

pub fn file_picker(file_name: &str) -> Vec<Vec<String>> {
    // Reading file from the root
    let file: String = fs::read_to_string(file_name)
    .expect("Error in reading {} file!");

    // Split array to level 1
    let splitted_file_string: Vec<String> = file.split_to_vec("\r");

    let mut final_value: Vec<Vec<String>> = Vec::new();
    for line in splitted_file_string {
        let block_as_string: Vec<String> = line.split_to_vec(" ");
        final_value.push(block_as_string);
    }

    return final_value;
}

trait QuickSplit { 
    // Getting only Vec of Strings
    fn split_to_vec(&self, separated_by: &'static str) -> Vec<String>;
}

impl QuickSplit for String {
    fn split_to_vec(&self, separated_by: &'static str) -> Vec<String> {
        return self.split(separated_by).map(|w| w.to_string()).collect();
    }
}