use std::io;

fn main() {
    easy_tuples();
    indexing_tuples();
    rust_arrays();
    invalid_array_access();
}

fn easy_tuples() {
    // Destructuring a tuple - this breaks the tuple into three equal parts
    let (x, y, z) = (500, 0.5, 1);
    println!("The value of x is {x}, y is {y}, and z is {z}");
}

fn indexing_tuples() {
    // Accessing tuple elements using a period "."
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The first value in the tuple is {five_hundred}, followed by {six_point_four} then finally {one}");
}

fn rust_arrays() {
    // Rust arrays are of fixed size, in contrast to vectors which are more flexible
    let a = [0, 1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The array has {first} at the beginning, followed by {second}");

    // Initializing with repeating elements
    let five_five_times = [5; 5];
    println!("Five five times: {five_five_times:#?}");
}

fn invalid_array_access() {
    // If user provides a value larger than the array's index, the program with panic and exit.
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}