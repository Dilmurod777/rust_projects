use std::io;

pub fn get_single_integer(text: &str, has_bounds: bool, min: i32, max: i32) -> i32 {
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

pub fn get_vec_of_integers(text: &str, sort: bool) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    println!("{text}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input. Try again!");

    let values = input.trim().split(' ');

    for value in values {
        let number: i32 = match value.parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        numbers.push(number);
    }

    if sort {
        numbers.sort();
    }

    return numbers;
}