fn main() {
    let rectangle = Rectangle {
        length: 10,
        width: 5,
    };
    let rectangle2 = Rectangle {
        length: 9,
        width: 4,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    println!(
        "Can the second rectangle fit in the first? {}",
        rectangle.can_hold(&rectangle2)
    );

    // Debugging
    println!("\nRectangle: {:#?}", &rectangle);

    // Takes ownership of an expression, as opposed to println! which takes a reference
    dbg!(&rectangle);

    // This block shows how associated functions work
    let square = Rectangle::square(5);
    dbg!(&square);
    println!("Area of the square is {}", square.area());
}

#[derive(Debug)] //needs to implemented above the object we want to debug
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size,
        }
    }
}
