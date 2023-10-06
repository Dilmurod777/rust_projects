use std::fs;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::input;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    output_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 0 {
            return Err("Invalid command. Try again!");
        } else if args[0] != "minigrep" {
            return Err("Command should start with `minigrep`. Try again!");
        } else if args.len() != 3 && args.len() != 5 {
            return Err("Invalid number of arguments. Try again!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let output_path = if args.len() == 5 {
            args[4].clone()
        } else {
            String::from("")
        };

        Ok(Config { query, file_path, ignore_case, output_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

pub fn save_to_file(output_path: &str, results: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_path)?;

    for line in results {
        file.write_fmt(format_args!("{}\n", line))?;
    }

    return Ok(());
}

pub fn run() {
    let command = input::get_simple_string("Enter command with following template: `minigrep searchstring filename.txt`");
    let parts: Vec<String> = command.trim().split_whitespace().map(str::to_string).collect();

    let config = match Config::build(&parts) {
        Err(err) => {
            eprintln!("{err}");
            return;
        }
        Ok(c) => c
    };

    let contents = match fs::read_to_string(config.file_path) {
        Err(_) => {
            eprintln!("No such file found. Try again!");
            return;
        }
        Ok(c) => c
    };

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if config.output_path != "" {
        match save_to_file(&config.output_path, results) {
            Err(_) => {
                eprintln!("Could not save to file {}", config.output_path);
            }
            _ => ()
        }
    } else {
        for line in results {
            println!("{line}")
        }
    }
}