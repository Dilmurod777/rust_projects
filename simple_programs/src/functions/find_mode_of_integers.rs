use std::collections::HashMap;
use crate::input;

pub fn run() {
    let numbers = input::get_vec_of_integers("Enter list of integers separated by space:", true);
    let l = numbers.len();

    if l == 0 {
        println!("List is empty.");
        return;
    }

    let mut freq = HashMap::new();
    let mut modes: Vec<i32> = Vec::new();
    let mut max_freq: i32 = 0;

    for number in numbers {
        let v = freq.entry(number).or_insert(0);
        *v += 1;

        if *v >= max_freq {
            max_freq = *v;
        }
    }

    for number in freq.keys() {
        if freq[number] == max_freq {
            modes.push(*number);
        }
    }

    if modes.len() == 1 {
        println!("Mode is {}", modes[0]);
    } else {
        modes.sort();
        println!("Several modes found: {:?}", modes);
    }
}