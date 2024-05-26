use std::io;

fn main() {
    println!("Enter a number");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: u32 = value
        .trim()
        .parse()
        .expect("value entered was not a number");

    let x = plus_one(value);
    println!("Final value is {x}");
}

fn plus_one(x: u32) -> u32 {
    x + 1
}
