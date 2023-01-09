use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = parse_config(&args);

    println!("Searching for string '{}'", config.query);
    println!("In file: '{}'", config.file_path);

    let contents: String = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let file_path: String = args[2].clone();

    Config { query, file_path }
}