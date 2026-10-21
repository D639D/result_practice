use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("Hello.txt")
        .expect("Не удалось открыть файл!");
}
