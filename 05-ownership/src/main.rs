// Rust memory model is unlike other paradigms:
// No manual management and no garbage collector.
// Memory is managed through a sysytem of ownership which is checked compile time
// by the compiler.

// In Rust, it matters if data is stored in heap or stack.

// RULES
//  - Each value in Rust has a variable that’s called its owner.
//  - There can only be one owner at a time.
//  - When the owner goes out of scope, the value will be dropped.

fn main() {
    scope_test();
    stack_assignment();
    heap_pointer_test();
    heap_clone();

    let s = String::from("acid");
    let z = give_me_data(s);
    // println!("This is an error {}", s);
    println!("Data is now available again: {}", z);

    let measured_string = get_string_length(z);
    println!("{} is {} characters long", measured_string.0, measured_string.1);

    let y = String::from("I'll be passed as reference");
    println!("'{}' is {} characters long", y, get_string_length_reference(&y));

    // Mutable reference
    let mut agent = String::from("Joe");
    mutate_with_reference(&mut agent);
    println!("Agent is: {}", agent);

}

fn scope_test() {
    let _msg = "dope";

    {
        let _msg = String::from("even doper"); // heap based
        println!("This is {}", _msg);
    }
    println!("This is {}", _msg);
}

fn stack_assignment() {
    let mut x = 10;
    let y = x;

    println!("x is {} and y is {}", x, y);

    x = 42;

    println!("x is {} and y is {}", x, y);

}

fn heap_pointer_test() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Throws an error, because s1 is _moved_ to s2;
    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}

fn heap_clone() {
    let city = String::from("Budapest");
    let hometown = city.clone();

    if city == hometown {
        println!("It's the same city");
    }
}

fn give_me_data(data: String) -> String {
    println!("I see data: {}", data);
    data
}

fn get_string_length(s: String) -> (String, usize) {
    let len = s.len();
    // Return a tuple
    (s, len)
}

fn get_string_length_reference(s: &String) -> usize {
    s.len()
}

fn mutate_with_reference(incoming: &mut String) {
    incoming.push_str(" Black");
}

// Defining this function results in panic
// There can't be a reference in block that's lifetime is not realted to main()
// References can't exist in a vacuum

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

