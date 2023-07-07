// setup clippy
#![deny(clippy::all)]
fn main() {
    println!("Hello, Rust!");
    variables();
    ownership();
    functions();
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

fn functions() {
    println!("----------functions----------");
    let hello = get_hello(String::from("Rust"));
    println!("{}", hello);
    say_hello();

    // inline functions
    let say_goodbye = |name: String| println!("Goodbye, {}!", name);
    say_goodbye(String::from("Rust"));
    let cal = |x: i32, y: i32| {
        let sum = x + y;
        let diff = x - y;
        sum * diff
    };
    println!("(x+y)(x-y)= {}", cal(10, 5));

    // callback
    fn fn_need_callback(x: i32, y: i32, callback: fn(i32) -> ()) {
        callback(x + y);
    }
    fn_need_callback(10, 5, |sum| println!("sum = {}", sum));
}
fn get_hello(to_person: String) -> String {
    format!("hello {}!", to_person)
}

// Unit type in Rust is ()
// fn say_hello() -> () {
fn say_hello() {
    println!("Hello, Rust!");
}
