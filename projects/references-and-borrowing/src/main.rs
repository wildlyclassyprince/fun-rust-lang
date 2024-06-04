fn main() {
    // References allow us to refer to values without taking ownership of them.
    // But since we do not have ownership of the variable we are referring to, it is not dropped.
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change_string(&mut s1);
    println!("{}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}
