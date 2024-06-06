fn main() {
    let rectangle = (5, 10);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
