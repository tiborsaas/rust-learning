fn main() {
    let s = String::from("This is the real thing");
    let fi = first_word(&s);

    println!("Full string: '{}', first word end index: {}", s, fi);

    let fword = &s[0..fi];
    println!("First word is: {}", fword);
}

/**
 * Calculates the first word's index
 */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == 32 {
            return i;
        }
    }
    s.len()
}
