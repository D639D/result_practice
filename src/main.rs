use std::{error, fs::File};

fn main() {
    let greeting_file_result = File::open("Hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}"),
    };
}
