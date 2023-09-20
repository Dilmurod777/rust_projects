use crate::input;

pub fn run() {
    let numbers = input::get_vec_of_integers("Enter list of integers separated by space:", true);
    let l = numbers.len();

    if l == 0 {
        println!("List is empty.");
        return;
    }

    if l % 2 == 0 {
        println!("Median is {:.1}", (numbers[l / 2] + numbers[l / 2 - 1]) as f32 / 2.0);
    } else {
        println!("Median is {}", numbers[l / 2]);
    }
}