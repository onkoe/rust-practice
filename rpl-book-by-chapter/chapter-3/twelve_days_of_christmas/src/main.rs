const LYRICS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans-a-swimming",
    "eight maids-a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    // i have no idea why its 13... wtf
    for days in 0..13 {
        print_from_number(days);
    }
}

fn print_from_number(days: usize) {

    match days {
        0 => {
            println!("On the first day of Christmas my true love sent to me...");
            println!("{}", LYRICS[days]);
            println!();
            return;
        }
        1 => {
            return;
        }
        _ => {
            println!("On the first day of Christmas my true love sent to me...");
            for day in (1..days).rev() {
                println!("{}, ", LYRICS[day]);
            }
            println!("and {}!", LYRICS[0]);
        }
    }

    println!();
}
