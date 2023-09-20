use crate::input;

pub fn run() {
    let n = input::get_single_integer("Enter value for n (1-1000):", true, 1, 1001);
    let mut a = 0;
    let mut b = 1;
    let result: i32;

    if n == 1 {
        result = a;
    } else if n == 2 {
        result = b;
    } else {
        for _ in 0..n - 2 {
            let temp = a;
            a = b;
            b = temp + b;
        }

        result = b;
    }

    println!("{n}-th Fibonacci number is {result}");
}