fn main() {
    let mut x = 42;
    println!("Logging a integer, {}", x);

    x = 32;
    println!("Logging a integer which is overwritten, {}", x);

    shadow();
    data_types();
}

fn shadow() {
    let x = 0;
    let x = x + 12;

    // Shadowing is different from marking a variable as mut,
    // because we’ll get a compile-time error if we accidentally try
    // to reassign to this variable without using the let keyword.
    // By using let, we can perform a few transformations on a value
    // but have the variable be immutable after those transformations
    // have been completed.
    println!("Shadowing test, {}", x);
}

fn data_types() {
    let x: u8 = 120;
    println!("Logging a 8 bit unsigned integer, {}", x);

    // u means unsigned and i is signed

    let minus: i8 = -120;
    println!("Logging a negative 8 bit integer, {}", minus);

    // let x: u8 = 314;
    // This will throw error:
    // error: literal out of range for u8
    // println!("Logging a 8 bit unsigned integer, {}", x);

    let my_color = 0xffffff;
    println!("Logging a hexadecimal number literal, {}", my_color);

    let my_binary: u8 = 0b00000011;
    println!("Logging a binary number literal, {}", my_binary);

    // Floating-point numbers are represented according to the IEEE-754 standard.
    // The f32 type is a single-precision float, and f64 has double precision.
    let float: f32 = 0.2;
    println!("Logging a floating point number, {}", float);

    let float2: f32 = 0.1;
    // yields 0.3 sharp :)
    println!("Running the classic float test, {}", float + float2);

    let is_bool: bool = true;
    println!("This is bool, {}", is_bool);

    // Chars can handle single characters, but way more than ASCII, it handles Unicode
    let char_type: char = 't';
    let unicode = '😻';
    println!("Sigle characters, {}, {}", char_type, unicode);

    // Tuples have a fixed length.
    // Once declared, they cannot grow or shrink in size.
    let color_tuple: (u8, u8, u8, bool) = (255, 255, 255, false);
    println!("Print color_tuple, {:?}", color_tuple);
    println!("Pretty print color_tuple, {:#?}", color_tuple);

    // Tuple destructuring
    let tomato_color = (255, 99, 71);
    let (r, g, b) = tomato_color;
    println!("Destructured, {}, {}, {}", r, g, b);
    println!("Direct access to tuple, {}", tomato_color.0);

    // ARRAYS
    // - Unlike a tuple, every element of an array must have the same type
    // - have a fixed length, like tuples
    // - data is allocated on stack

    let arr = [1,2,3,4,5,6];
    println!("Array is: {:?}", arr);

    // explicit declaration of an array
    let awe: [char; 4] = ['s', 'o', 'm', 'e'];
    println!("Array is: {:?}", awe);

}