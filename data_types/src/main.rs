fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let x = 2.0; // f64
    println!("{x}");

    let y: f32 = 3.0; // f32
    println!("{y}");

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let truncated2 = 5 / 3;
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("truncated2: {}", truncated2);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    // boolean
    let t = true;
    let f: bool = false;
    println!("{t}");
    println!("{f}");

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eye_cat = '😻';
    println!("{c}");
    println!("{z}");
    println!("{heart_eye_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of x is: {z}");

    let five_hundred = tup.0;
    println!("{five_hundred}");
    println!("{}", tup.1);
    println!("{}", tup.2);

    // array
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
}
