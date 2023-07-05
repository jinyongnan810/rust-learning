// setup clippy
#![deny(clippy::all)]
fn main() {
    println!("Hello, Rust!");
    variables();
    ownership()
}

fn variables() {
    println!("----------variables----------");
    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    // changeable variables & immutable variables
    let first_name = "Rust";
    let mut last_name = "Lang";
    let age = 18;
    println!(
        "My name is {} {}, I'm {} years old.",
        first_name, last_name, age
    );
    last_name = "Language";
    println!(
        "My name is {} {}, I'm {} years old.",
        first_name, last_name, age
    );
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    println!("tup.0 = {}", tup.0);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn ownership() {
    println!("----------ownership----------");
    let name = String::from("Rust");
    // name is "moved" to name2
    let name2 = name;
    // cannot use name anymore
    // println!("name = {}", name);
    println!("name2 = {}", name2);
    greet(&name2);
    // using pointer to reference without "moving"
    let name3 = &name2;
    println!("name3 = {}", name3);
    // mutable reference
    let mut garbage = String::from("garbage");
    empty_string(&mut garbage);
    println!("garbage = {}", garbage);

    let age = 18;
    let age2 = age;
    println!("age = {}", age);
    println!("age2 = {}", age2);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}
fn empty_string(garbage: &mut String) {
    garbage.clear();
}
