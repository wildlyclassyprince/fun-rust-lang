fn main() {
    // write a program that calculates the area of a rectangle.
    let width_1 = 30;
    let height_1 = 50;

    println!("The are of the rectangle is {} square pixels.", area(width_1, height_1));

    // refactor with tuples
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels. (tuple refactor)", area_tuple(rect1));

    // refactor with structs
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels. (struct refactor)", area_struct(&rect2));
    println!("The struct values are:\n{rect2:#?}");
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    // borrows the Rectangle struct, doesn't take ownership
    rectangle.width * rectangle.height
}
