use std::fs;

use crate::input;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query,
            file_path,
        }
    }
}


pub fn run() {
    let command = input::get_simple_string("Enter command with following template: `minigrep searchstring filename.txt`");
    let parts: Vec<String> = command.trim().split_whitespace().map(str::to_string).collect();

    if parts.len() == 0 {
        println!("Invalid command. Try again!");
        return;
    } else if parts[0] != "minigrep" {
        println!("Command should start with `minigrep`. Try again!");
        return;
    } else if parts.len() != 3 {
        println!("Invalid number of arguments. Try again!");
        return;
    }

    let config = Config::new(&parts);

    let contents = match fs::read_to_string(config.file_path) {
        Err(_) => {
            println!("No such file found. Try again!");
            return;
        }
        Ok(c) => c
    };

    println!("File contents: {contents}");
}