fn main() {
    simple_example();
    string_example();
    ownership_and_functions();
    return_values_and_scope();
    returning_ownership_of_parameters();
}

fn simple_example() {
    // Bind 5 to x; then make a copy of the value in x and bind it to y.
    // These variables use only the stack becuase they have a known size at compile time.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}

fn string_example() {
    // String types do not work like string literals.
    // Allocate memory in the heap for storing the string "Hello".
    // The memory address is stored in the stack as a pointer and bound to s1.
    // Copying s1 to s2 copies the pointer not the value in memory, meaning two variables point to the same location in memory.
    // This creates a double free error, an undesireable memory safety bug.
    // To copy the value of s1 to s2 we need to use 'clone()'.
    // These variables use the heap (because the size is not known at compile time) and stack (to store the address of the memory allocation).
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}

fn ownership_and_functions() {
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("Hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("Takes ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Makes copy: {}", some_integer);
}

fn return_values_and_scope() {
    let _s1 = gives_ownership();

    let s2 = String::from("Hello");

    let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn returning_ownership_of_parameters() {
    // We can return multiple values using tuples.
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
