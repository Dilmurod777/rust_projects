use crate::garden::vegetables::Asparagus;

pub mod garden;

fn reverse(input: &str) -> String {
    let mut output = String::new();
    let l = input.len();

    for i in input.chars().rev() {
        output.push(i)
    }

    return output;
}

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    let r = reverse("子猫");
    println!("{r}");
}