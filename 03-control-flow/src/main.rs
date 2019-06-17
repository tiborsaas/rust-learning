fn main() {
    let x = 55;

    if x < 10 {
        println!("Hello, world!");
    } else {
        println!("X is bigger than 10");
    }

    // if x {
    //     println!("We are not in JavaScript land anymore dude");
    // }

    if x != 0 {
        println!("We are not in JavaScript land anymore :)");
    }

    conditional_assignment();
    loop_around(8);
    loop_with_while();
    print_collection_while();
    print_collection_for_loop();
}

fn conditional_assignment() {
    let condition = true;

    // The returned values must be the same type
    let birth_year = if condition != false {
        1983
    } else {
        1000
    };

    println!("I was born in: {}", birth_year);
}

fn loop_around(times: i32) {
    let mut c = 0;
    let result = loop {
        println!("Counter is {}", c);
        c += 1;

        if c == times {
            break c;
        }
    };
    println!("Counter is evaluated finally to {}", result);
}

fn loop_with_while() {
    let mut num = 5;

    while num != 0 {
        println!("Weee, going down: {}", num);
        num -= 1;
    }
    println!("BANG!");
}

fn print_collection_while() {
    let favorite_days = ["Firday", "Saturday", "Sunday"];
    let mut i = 0;

    // This is slow, because the compiler adds runtime code
    // to perform the conditional check on every element on
    // every iteration through the loop
    while i < favorite_days.len() {
        println!("Fav day is now: {}", favorite_days[i]);
        i += 1;
    }
}

fn print_collection_for_loop() {
    let favorite_days = ["Firday", "Saturday", "Sunday"];

    for day in favorite_days.iter() {
        println!("Still, my fav day is now: {}", day);
    }
}

