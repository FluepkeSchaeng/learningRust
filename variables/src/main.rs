use std::io;

fn main() {
    // example of a constant declaration
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // explaining of mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in the outer scope is: {y}");

    // floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // simple mathematic operations
    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multiplikation
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // boolean type
    let t = true;
    let f: bool = false; //with eyplicit type annotation

    // character type
    /* Rust's char type is four bytes in size and represents a Unicode
     *  Scalar Value, which means it can represent a lot more than ASCII.
     *  Accented letters; Chinese, Japanese and Korean characters; emoji;
     *  and zero-width spaces are all valid char values in Rust. */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compuound types (tuples and arrays)

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // also possible:
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    // accessing tuple elements directly
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array type
    let a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // you can also initialize an array to contain the same value for each element
    // by specifying the initial value, followed by a semicolon, and the the length
    // of the array in square brackets
    let _a = [3; 5];
    // -> the same as 'let a = [3, 3, 3, 3, 3];' but more concise
    let _first = a[0];
    let _second = a[1];

    // This snippet produces a runtime out of bound panic when entering
    // a number greater than 4
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
