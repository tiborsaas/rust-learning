fn main() {
    let s = String::from("This is the real thing");
    let fi = first_word(&s);

    println!("Full string: '{}', first word end index: {}", s, fi);

    let fword = &s[0..fi];
    println!("First word is: {}", fword);

    let text = String::from("The Doors");
    let from = 4;
    let to = text.len();
    println!("String range on String: {}", string_range(&text, from, to));
    println!("String range string literal: {}", string_range("The Houses", from, to));

    let coll = [4, 8, 15, 16, 23, 42];
    println!("Some lost number: {:?}", array_range(&coll, 2, 4));
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

fn string_range(user_string: &str, start: usize, end: usize) -> &str {
    &user_string[start..end]
}

fn array_range(arr: &[i32], start: usize, end: usize) -> &[i32] {
    &arr[start..end]
}
