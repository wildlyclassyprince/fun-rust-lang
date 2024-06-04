fn main() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    let s1 = String::from("hello world");
    let s2 = String::from("helloworld");
    let word1 = first_word(&s1);
    let word2 = first_word(&s2);

    println!("First word (with space): {}\nFirst word (without spaces): {}", word1, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}