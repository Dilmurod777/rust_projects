use crate::input;

const TEMPERATURE_NAMES: [&str; 2] = ["Fahrenheit", "Celsius"];
const TEMPERATURE_UNITS: [&str; 2] = ["F", "C"];

fn show_temperature_units() {
    for i in 0..TEMPERATURE_NAMES.len() {
        let unit = TEMPERATURE_NAMES[i];
        println!("{i} - {unit}");
    }
}

pub fn run() {
    println!("Select temperature unit to convert FROM:");
    show_temperature_units();
    let from_unit: i32 = input::get_numeric_input("Your choice:", true, 0, 2);

    println!("Select temperature unit to convert TO:");
    show_temperature_units();
    let to_unit: i32 = input::get_numeric_input("Your choice:", true, 0, 2);

    println!("Enter value to convert:");
    let value: f32 = input::get_numeric_input("Value:", false, -1, -1) as f32;
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