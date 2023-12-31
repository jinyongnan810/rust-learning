// setup clippy
#![deny(clippy::all)]

mod pointers;
fn main() {
    println!("Hello, Rust!");
    variables();
    ownership();
    functions();
    structures();
    enums();
    collections();
    optionals();
    handling_error();
    lifetimes();
    traits();
    pointers::pointers();
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

// allow debug for struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
// tuple struct
struct Position(i32, i32, i32);
// struct implementation
impl Position {
    fn describe(&self) {
        println!(
            "described as x = {}, y = {}, z = {}",
            self.0, self.1, self.2
        );
    }
    fn distance(&self, other: &Position) -> f64 {
        let x = (self.0 - other.0).pow(2);
        let y = (self.1 - other.1).pow(2);
        let z = (self.2 - other.2).pow(2);
        ((x + y + z) as f64).sqrt()
    }
    fn make_twice(&mut self) {
        self.0 *= 2;
        self.1 *= 2;
        self.2 *= 2;
    }
    fn zero() -> Position {
        Position(0, 0, 0)
    }
}
fn structures() {
    println!("----------structures----------");
    let mut person = Person {
        name: String::from("Rust"),
        age: 18,
        // impl Debug for Person
    };
    println!("name = {}, age = {}", person.name, person.age);
    println!("person = {:?}", person);
    person.age += 1;
    println!("name = {}, age = {}", person.name, person.age);
    let person2 = Person { age: 33, ..person };
    println!("name = {}, age = {}", person2.name, person2.age);
    // tuple struct
    let position = Position::zero();
    println!("x = {}, y = {}, z = {}", position.0, position.1, position.2);
    position.describe();
    let mut position2 = Position(1, 1, 1);
    println!("distance = {}", position.distance(&position2));
    position2.make_twice();
    println!("distance = {}", position.distance(&position2));
    // println!("position = {:?}", position);
}

// note the naming convention
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum AnimalType {
    Dog,
    Cat,
    Bird,
}

enum Shape {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}
impl Shape {
    fn area(&self) -> f64 {
        // match returns value
        match self {
            Shape::Circle { radius, .. } => std::f64::consts::PI * radius.powi(2),
            Shape::Rectangle { width, height } => width * height,
        }
    }
}
fn enums() {
    println!("----------enums----------");
    let animal = AnimalType::Dog;
    println!("animal = {:?}", animal);
    println!("is dog? {}", animal == AnimalType::Dog);
    match animal {
        AnimalType::Dog => println!("dog"),
        AnimalType::Cat => println!("cat"),
        // AnimalType::Bird => println!("bird"),
        _ => println!("some other"),
    }

    let shape1 = Shape::Circle {
        radius: 2.0,
        center: (0.0, 0.0),
    };
    let shape2 = Shape::Rectangle {
        width: 2.0,
        height: 2.0,
    };
    match shape1 {
        Shape::Circle { radius, center } => {
            println!("circle with radius {} at {:?}", radius, center)
        }
        Shape::Rectangle { width, height } => {
            print!("rectangle with width {} and height {}", width, height)
        }
    }
    if let Shape::Rectangle { width, height } = shape2 {
        println!("rectangle with width {} and height {}", width, height)
    }
    println!("area = {}", shape1.area());
    println!("area = {}", shape2.area());
}

use std::collections::HashMap;
fn collections() {
    println!("----------collections----------");
    // tuples
    let person = ("Rust", 18);
    println!("name = {}, age = {}", person.0, person.1);
    let (name, age) = person;
    println!("name = {}, age = {}", name, age);

    // vectors(like list/array)
    let values = [4.2, 0.5];
    for value in values.iter() {
        println!("value = {}", value);
    }
    let new_values = values.iter().map(|value| value * 2.2);
    for value in new_values {
        println!("value = {}", value);
    }
    // size changeable vector
    // can also create by let mut numbers = Vec::new();, but it's not recommended to push right after it.
    let mut numbers: Vec<i32> = vec![0];
    numbers.push(1);
    numbers.push(2);
    numbers.extend_from_slice(&[3, 4, 5]);
    println!("numbers = {:?}", numbers);
    let mut another_numbers = vec![6];
    numbers.append(&mut another_numbers);
    println!("numbers = {:?}", numbers);
    println!("another_numbers = {:?}", another_numbers);
    println!("numbers has 5? {}", numbers.contains(&5));
    println!("numbers is empty? {}", numbers.is_empty());
    println!("another numbers is empty? {}", another_numbers.is_empty());

    // hashmap
    let mut map = HashMap::new();
    map.insert("hello", "world");
    println!("map = {:?}", map);
    println!("map[hello] = {:?}", map.get("hello"));
    println!("contains hello? {}", map.contains_key("hello"));
    match map.get("hello1") {
        Some(value) => println!("value = {}", value),
        None => println!("no value"),
    }
    for (&key, &value) in &map {
        println!("key = {}, value = {}", key, value);
    }
    map.entry("hello").or_insert("world1");
    println!("map = {:?}", map);

    // Iterators
    for value in numbers.iter() {
        println!("value = {}", value);
    }
    println!("sum = {}", numbers.iter().sum::<i32>());
    numbers
        .iter()
        .map(|x| x * 2)
        .for_each(|x| println!("x = {}", x));
    let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("doubled = {:?}", doubled);
    let even = numbers
        .iter()
        .filter(|x| **x % 2 == 0)
        .collect::<Vec<&i32>>();
    println!("even = {:?}", even);
}

fn optionals() {
    println!("----------optionals----------");
    let mut maybe_number: Option<i32> = None;
    println!("maybe_number = {:?}", maybe_number);
    // safe
    println!("maybe_number default = {}", maybe_number.unwrap_or(10));
    maybe_number = Some(42);
    println!("maybe_number = {:?}", maybe_number);
    match maybe_number {
        Some(number) => println!("number + 10 = {}", number + 10),
        None => println!("no number"),
    }
    let maybe_number2: Option<i32> = None;
    if let Some(number) = maybe_number2 {
        println!("maybe_number2 = {}", number);
    } else {
        println!("no number");
    }

    // unsafe
    let unwraped_number = maybe_number.expect("no number");
    println!("unwraped_number = {}", unwraped_number);
    let force_unwraped_number = maybe_number.unwrap();
    println!("force_unwraped_number = {}", force_unwraped_number);
    // mutate
    if let Some(number) = maybe_number.as_mut() {
        *number += 10
    }
    println!("maybe_number = {:?}", maybe_number);
    // unwrap multiple
    let maybe_number2 = Some(22);
    let maybe_number3 = Some(33);
    if let (Some(number2), Some(number3)) = (maybe_number2, maybe_number3) {
        println!("number2 + number3 = {}", number2 + number3);
    }
}

fn get_first_name() -> Result<String, ()> {
    // Ok("Rust".to_string())
    Err(())
}
fn get_last_name() -> Result<String, ()> {
    println!("get_last_name");
    Ok("lang".to_string())
}
fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}
fn handling_error() {
    println!("----------handling error----------");
    let ok_value: Result<&str, Box<dyn std::error::Error>> = Ok("hello");
    match ok_value {
        Ok(value) => println!("value = {}", value),
        Err(error) => println!("error = {}", error),
    }

    let ng_value: Result<&str, Box<dyn std::error::Error>> = Err("something when wrong...".into());
    match ng_value {
        Ok(value) => println!("value = {}", value),
        Err(error) => println!("error = {}", error),
    }

    let void_ng_value: Result<&str, ()> = Err(());
    match void_ng_value {
        Ok(value) => println!("value = {}", value),
        Err(error) => println!("error = {:?}", error),
    }

    let expect_value: Result<&str, ()> = Ok("hello"); //Err(());
    println!(
        "expect_value = {}",
        expect_value.expect("something when wrong...")
    );

    let check_ok_value: Result<&str, ()> = Ok("hello");
    println!("is ok? {}", check_ok_value.is_ok());

    let full_name = get_full_name();
    match full_name {
        Ok(value) => println!("full name = {}", value),
        Err(error) => println!("get full name error = {:?}", error),
    }
    let length = get_full_name().map(|name| name.len()).unwrap_or_default();
    println!("length = {}", length);
}

// fn get_string_slice() -> &str {
//     "hello"
// }
fn get_string_slice() -> &'static str {
    "hello"
}
// general lifetime annotation
fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
struct PersonWithName<'l> {
    name: &'l str,
}
// lifetime elision for single input
fn get_person_name(name: &str) -> &str {
    name
}
impl PersonWithName<'_> {
    fn get_name(&self) -> &str {
        self.name
    }
}
fn lifetimes() {
    println!("----------lifetimes----------");
    println!("get_string_slice = {}", get_string_slice());
    println!("get_random_name = {}", get_random_name("rust", "lang"));
    let person = PersonWithName { name: "rust" };
    println!("person name = {}", person.name);
    println!("get_person_name = {}", get_person_name("rust"));
    println!("person get_name = {}", person.get_name());
}

// define trait and implement it
trait Greeter {
    fn greet(&self);
}
impl Greeter for PersonWithName<'_> {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}
// trait that creates new instance
trait CanCreateNew {
    fn new(name: String, age: u8) -> Self;
}
impl CanCreateNew for Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
}

use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name = {}, age = {}", self.name, self.age)
    }
}

fn print_greet(greeter: &impl Greeter) {
    greeter.greet();
}
fn print_greet2<T: Greeter>(greeter: &T) {
    greeter.greet();
}

trait Goodbye {
    fn goodbye(&self);
}
impl Goodbye for PersonWithName<'_> {
    fn goodbye(&self) {
        println!("Goodbye, {}!", self.name);
    }
}
// multiple traits
fn print_greet_and_goodbye<T: Greeter + Goodbye>(greeter: &T) {
    greeter.greet();
    greeter.goodbye();
}
fn print_greet_and_goodbye2<T>(greeter: &T)
where
    T: Greeter + Goodbye,
{
    greeter.greet();
    greeter.goodbye();
}

// trait of trait
struct Person2 {
    first_name: String,
    last_name: String,
}
trait HasName {
    fn get_first_name(&self) -> &str;
    fn get_last_name(&self) -> &str;
}
impl HasName for Person2 {
    fn get_first_name(&self) -> &str {
        &self.first_name
    }
    fn get_last_name(&self) -> &str {
        &self.last_name
    }
}
trait FullName
where
    Self: HasName,
{
    fn get_full_name(&self) -> String;
}
impl<T> FullName for T
where
    T: HasName,
{
    fn get_full_name(&self) -> String {
        format!(
            "full name is: {} {}",
            self.get_first_name(),
            self.get_last_name()
        )
    }
}

fn traits() {
    println!("----------traits----------");
    let person = PersonWithName { name: "rust" };
    person.greet();
    let person2 = Person::new("rust".to_string(), 18);
    println!("person2 = {:?}", person2);
    // impl std::fmt::Display
    println!("person2 = {}", person2);
    // trait as parameter
    print_greet(&person);
    print_greet2(&person);
    print_greet_and_goodbye(&person);
    print_greet_and_goodbye2(&person);
    // trait of trait
    let person3 = Person2 {
        first_name: "rust".to_string(),
        last_name: "lang".to_string(),
    };
    println!("person3 = {}", person3.get_full_name());
}
