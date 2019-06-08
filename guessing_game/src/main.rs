use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number please!");
    println!("---");
    println!("Input guess.");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number dude!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That's smaller."),
            Ordering::Greater => println!("That's greater."),
            Ordering::Equal => {
                println!("Finally... :)");
                break;
            }
        }
    }
}
