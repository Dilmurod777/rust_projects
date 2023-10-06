use dotenv::dotenv;

pub mod input;
pub mod functions;

pub mod tests;

const MENU_OPTIONS: [&str; 9] = [
    "Exit",
    "Guess the number",
    "Convert temperature",
    "Generate n-th Fibonacci number",
    "Print Christmas Carol Lyrics",
    "Reverse string",
    "Find median of integers",
    "Find mode of integers",
    "Minigrep"
];

fn main() {
    dotenv().ok();

    loop {
        println!("Choose the program:");
        show_menu();
        let choice: i32 = input::get_single_integer("Your choice:", true, 0, MENU_OPTIONS.len() as i32);

        if choice == 0 {
            println!("Successfully exited. See you next time!");
            break;
        } else if choice == 1 {
            functions::guess_number_game::run();
        } else if choice == 2 {
            functions::convert_temperature::run();
        } else if choice == 3 {
            functions::generate_fibonacci_numbers::run();
        } else if choice == 4 {
            functions::christmas_carol_lyrics::run();
        } else if choice == 5 {
            functions::reverse_string::run();
        } else if choice == 6 {
            functions::find_median_of_integers::run();
        } else if choice == 7 {
            functions::find_mode_of_integers::run();
        } else if choice == 8 {
            functions::minigrep::run();
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