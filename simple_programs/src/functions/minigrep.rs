use std::fs;

use crate::input;

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

    let search_str = &parts[1];
    let filename = &parts[2];

    println!("Searching for {}", search_str);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename);

    let contents = match contents {
        Err(_) => {
            println!("No such file found. Try again!");
            return;
        }
        Ok(c) => c
    };

    println!("File contents: {contents}");
}