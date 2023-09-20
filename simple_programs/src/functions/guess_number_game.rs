use rand::Rng;
use std::io;
use std::cmp::Ordering;

use crate::input;

pub fn run() {
    let min = 1;
    let max = 100;
    let secret_number = rand::thread_rng().gen_range(min..max);

    println!("Find the secret number between {min} and {max}.");

    let mut trials = 0;
    loop {
        let guess: i32 = input::get_single_integer("Your guess:", true, min, max);
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