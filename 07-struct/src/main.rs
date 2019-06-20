struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {

    let mut user_instance = User {
        email: String::from("tibor@szasz.hu"),
        username: String::from("tiborsaas"),
        active: true,
    };

    user_instance = updateEmail(user_instance, String::from("master@key.horse"));

    // If a field is never used, the compiler throws a warning which is pretty nice
    println!("User is {}", user_instance.email);
}

fn updateEmail(mut user: User, toEmail: String) -> User {
    user.email = toEmail;
    user
}
