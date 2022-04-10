use std::fs;
fn main() {
    file_picker("sample.txt");
}

pub fn file_picker(file_name: &str) {
    // Reading file from the root
    let file: String = fs::read_to_string(file_name).expect("Error in reading {} file!");
    
    // Printing the file content
    let splitted_file_string: Vec<String> = file.split("\r").map(|w| w.to_string()).collect();
    println!("{:?}", splitted_file_string);

    let mut final_value: Vec<Vec<String>> = Vec::new();
    for line in splitted_file_string {
        // println!("{}", line);
        let block_as_string: Vec<String> = line.split(" ").map(|w| w.to_string()).collect();
        final_value.push(block_as_string);
    }

    print!("{:?}", final_value[0][1]);
}