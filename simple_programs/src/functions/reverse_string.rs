use std::io;

pub fn run() {
    println!("Enter string:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input. Try again!");

    let input = input.trim();

    let mut output = String::new();

    for i in input.chars().rev() {
        output.push(i)
    }

    println!("Reverse of '{input}' is '{output}'.")
}