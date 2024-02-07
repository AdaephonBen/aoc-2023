use std::{
    fs::File,
    io::{BufReader,prelude::*},
    path::Path
};

pub fn read_file_into_vector(filename: String) -> Vec<String> {
    let file_path = Path::new(filename.as_str());
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open file{}, {}", file_path.display(), why),
        Ok(file) => file,
    };
    let buffer = BufReader::new(file);
    let vec: Vec<String> = buffer.lines().map(|l| l.expect("couldn't parse line")).collect();
    vec
}

pub fn substring(str: String, start_index: u32, end_index: u32) -> String {
    let mut final_substring: String;
    let chars = str.chars();
    for index in start_index..end_index {
        final_substring.push(chars.nth(index as usize).unwrap());
    }
    final_substring
}
