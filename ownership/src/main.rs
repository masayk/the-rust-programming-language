fn main() {
    let s = String::from("hello");

    takes_ownership(s); // move

    // println!("{s}"); // error

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let x = 5;
    let y = x;
    println!("{x}");
    println!("{y}");

    makes_copy(x);

    let s1 = String::from("hello");
    let s2 = s1; // move

    //println!("{s1}");
    println!("{s2}");

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3: {s3}, s4: {s4}");

    let s5 = gives_ownership();
    println!("s5: {s5}");

    let s6 = String::from("s6");
    let s7 = takes_and_gives_back(s6);
    println!("s7: {s7}");

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{s8}' is {len}.");

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{s9}");

    let s = String::from("hello world!");
    let word = first_word(&s);
    println!("{word}");

    let string_literal = "hello world!";
    let word = first_word(string_literal);
    println!("{word}");
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
