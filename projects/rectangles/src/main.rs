fn main() {
    let rectangle = Rectangle {
        length: 10,
        width: 5,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );

    // Debugging
    println!("\nRectangle: {:#?}", &rectangle);

    // Takes ownership of an expression, as opposed to println! which takes a reference
    dbg!(&rectangle);
}

#[derive(Debug)] //needs to implemented above the object we want to debug
struct Rectangle {
    length: u32,
    width: u32,
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.length * dimensions.width
}
