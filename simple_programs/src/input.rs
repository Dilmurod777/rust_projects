use std::io;

pub fn get_numeric_input(text: &str, has_bounds: bool, min: i32, max: i32) -> i32 {
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