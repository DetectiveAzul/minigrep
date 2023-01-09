use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (query, file_path) = parse_config(&args);

    println!("Searching for string '{}'", query);
    println!("In file: '{}'", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    (query, file_path)
}