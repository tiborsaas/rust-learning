fn setup() {
    // Rust doesn’t care where you define your functions
}

fn main() {
    println!("Hello, world!");
    setup();
    another_thing();
    get_some_params(2134);
    test_expressions();
    println!("Five is {} :)", five());
    println!("Six is {}", add_one(five()));
}

fn another_thing() {
    println!("I'm in another, world, but hello still!");
}

fn get_some_params(yolo: u16) {
    println!("We got {}", yolo);
}

fn test_expressions() {
    let expr = {
        let x = 3;
        x + 1 // < adding a semicolon turns this to a statment
        // statements have no return value
    };
    println!("expr evaluates to {:?}", expr);
}

fn five() -> i32 {
    5
}

fn add_one(num: i32) -> i32 {
    num + 1 // again, no semicolon for returning values
}
