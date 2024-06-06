fn main() {
    println!(
        "The area of the rectangle is {} square pixels.",
        area(5, 10)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
