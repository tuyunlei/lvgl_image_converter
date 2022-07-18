use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn load_file(filename: &str) -> File {
    match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error when open file: {}", e.to_string());
            exit(1)
        }
    }
}

fn main() {
    let file = load_file("config");
    let lines = BufReader::new(file).lines();

    for (i, line) in lines.into_iter().enumerate() {
        match line {
            Ok(line) => {
                let split: Vec<_> = line.split('=').collect();
                if split.len() != 2 {
                    println!("Error config line {}: {}", i, line);
                    exit(1);
                }
                let key = split[0].trim();
                let value = split[1].trim();
                println!("key={} value={}", key, value);
            }
            Err(e) => {
                println!("Error when read lines: {}", e.to_string());
                exit(1);
            }
        }
    }
}
