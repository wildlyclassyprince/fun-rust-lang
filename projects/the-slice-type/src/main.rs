fn main() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    let s1 = String::from("hello world");
    let s2 = String::from("helloworld");
    let word1 = first_word(&s1);
    let word2 = first_word(&s2);
    let word1bytes = first_word_bytes(&s1);
    let word2bytes = first_word_bytes(&s2);
    let secondword1 = second_word(&s1);
    let secondword2 = second_word(&s2);

    println!(
        "First word (with space): {}\nFirst word (without spaces): {}",
        word1, word2
    );

    println!(
        "First word bytes (with space): {}\nFirst word bytes (without space): {}",
        word1bytes, word2bytes
    );

    println!(
        "Second word (with space): {}\nSecond word (without space): {}",
        secondword1, secondword2
    );
}

fn first_word_bytes(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
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

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}
