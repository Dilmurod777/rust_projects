use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MENU_OPTIONS: [&str; 4] = ["Exit", "Guess the number", "Convert temperature", "Generate n-th Fibonacci number"];
const TEMPERATURE_NAMES: [&str; 2] = ["Fahrenheit", "Celsius"];
const TEMPERATURE_UNITS: [&str; 2] = ["F", "C"];

fn main() {
    loop {
        println!("Choose the program:");
        show_menu();
        let choice: i32 = get_numeric_input("Your choice:", true, 0, MENU_OPTIONS.len() as i32);

        if choice == 0 {
            println!("Successfully exited. See you next time!");
            break;
        } else if choice == 1 {
            guess_number_game();
        } else if choice == 2 {
            convert_temperature();
        } else if choice == 3 {
            generate_fibonacci_number();
        } else {
            println!("Invalid choice. Try again!")
        }
    }
}

fn show_menu() {
    for i in 0..MENU_OPTIONS.len() {
        let option = MENU_OPTIONS[i];
        println!("{i} - {option}");
    }
}

fn get_numeric_input(text: &str, has_bounds: bool, min: i32, max: i32) -> i32 {
    loop {
        println!("{text}");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input. Try again!");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        };

        if has_bounds {
            if input < min {
                println!("Input must be larger than {min}. Try again!");
                continue;
            }

            if input > max - 1 {
                println!("Input must be smaller than {max}. Try again!");
                continue;
            }
        }

        return input;
    }
}

fn guess_number_game() {
    let min = 1;
    let max = 100;
    let secret_number = rand::thread_rng().gen_range(min..max);

    println!("Find the secret number between {min} and {max}.");

    let mut trials = 0;
    loop {
        let guess: i32 = get_numeric_input("Your guess:", true, min, max);
        trials += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Guess larger!");
            }
            Ordering::Equal => {
                println!("Good game! You found the secret number {secret_number} in {trials} trials.");
                let _ = io::stdin();
                break;
            }
            Ordering::Greater => {
                println!("Guess smaller!")
            }
        }
    }
}

fn show_temperature_units() {
    for i in 0..TEMPERATURE_NAMES.len() {
        let unit = TEMPERATURE_NAMES[i];
        println!("{i} - {unit}");
    }
}

fn convert_temperature() {
    println!("Select temperature unit to convert FROM:");
    show_temperature_units();
    let from_unit: i32 = get_numeric_input("Your choice:", true, 0, 2);

    println!("Select temperature unit to convert TO:");
    show_temperature_units();
    let to_unit: i32 = get_numeric_input("Your choice:", true, 0, 2);

    println!("Enter value to convert:");
    let value: f32 = get_numeric_input("Value:", false, -1, -1) as f32;
    let mut converted_value = 0.0;

    if from_unit == to_unit {
        converted_value = value;
    } else if from_unit == 0 {
        if to_unit == 1 {
            converted_value = 5.0 / 9.0 * (value - 32.0);
        }
    } else if from_unit == 1 {
        if to_unit == 0 {
            converted_value = value * 9.0 / 5.0 + 32.0;
        }
    }

    let to_unit: &str = TEMPERATURE_UNITS[to_unit as usize];
    println!("Your converted value is {converted_value}{to_unit}")
}

fn generate_fibonacci_number() {
    let n = get_numeric_input("Enter value for n (1-1000):", true, 1, 1001);
    let mut a = 0;
    let mut b = 1;
    let result: i32;

    if n == 1{
        result = a;
    }else if n == 2{
        result = b;
    }else{
        for _ in 0..n-2{
            let temp = a;
            a = b;
            b = temp + b;
        }

        result = b;
    }

    println!("{n}-th Fibonacci number is {result}");
}