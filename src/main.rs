use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let content = fs::read_to_string(config.filename)
        .expect("Error reading the file");

    println!("File content:\n{}", content);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let filename = args[2].clone();

        Self {query, filename}
    }
}
