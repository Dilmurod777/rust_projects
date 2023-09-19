const CHRISTMAS_CAROL_PARTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtles doves", "three french hens",
    "The four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"];

const NUMBERS_TEXT_FORM: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

pub fn run() {
    let mut lyrics = String::new();
    let total_days = 12;

    for day in 0..total_days {
        lyrics = format!("{}On the {} day of Christmas, my true love sent to me\n", lyrics, NUMBERS_TEXT_FORM[day]);
        let parts = CHRISTMAS_CAROL_PARTS[0..day + 1].iter().rev();

        let mut index = 0;
        for part in parts {
            let extra = if day > 0 && index == day - 1 { ", and" } else { "" };
            lyrics = format!("{lyrics}{part}{extra}\n");
            index += 1;
        }

        lyrics = format!("{}\n", lyrics);
    }

    println!("{lyrics}");
}