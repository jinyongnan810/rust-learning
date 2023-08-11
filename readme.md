# Learning from

https://www.youtube.com/watch?v=-TFH38LYmvo&list=PL6yRaaP0WPkWRsXJgdnw9lj1vchAaKwfS

# Initial

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new first
rustup component add clippy

cargo install cargo-watch
cargo-watch -qc -x run -x clippy
```

# Variables

- Only `snake_case_variable` is valid.
- `let` is unchangeable, `let mut` is changeable

# Ownership

- `name` is became not accessible for [this](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation) reason

```rust
let name = String::from("Rust");
// name is "moved" to name2
let name2 = name;
println!("name = {}", name);
println!("name2 = {}", name2);
```

- using reference to avoid moving. ![ref](https://doc.rust-lang.org/book/img/trpl04-05.svg)

```rust
let name3 = &name2;
println!("name3 = {}", name3);

// moving also happens when take String as params
// the variable send to this function will be moved and cannot be accessed again
fn greet(name: String) {
    println!("Hello, {}!", name);
}
// so usually we make params with reference type
fn greet(name: &String) {
    println!("Hello, {}!", name);
}

// &String is immutable, to make it mutable, use &mut String
// memo: mutable reference is allowed only one at a time
fn empty_string(garbage: &mut String) {
    garbage.clear();
}

// borrow neighbor's car example
```

# Functions

```rust
// inline functions and callback functions
fn fn_need_callback(x: i32, y: i32, callback: fn(i32) -> ()) {
    callback(x + y);
}
fn_need_callback(10, 5, |sum| println!("sum = {}", sum));
```

# Structures
```rust
// make a structure and print
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
let mut person = Person {
    name: String::from("Rust"),
    age: 18,
    // impl Debug for Person
};
println!("person = {:?}", person);
// make structure functions using impl
struct Position(i32, i32, i32); // tuple structure
impl Position {
    fn xxx(self) {
        // do something...
    }
}
```

# Enums
```rust
// simple enum
enum AnimalType {
    Dog,
    Cat,
    Bird,
}
// enum with data
enum Shape {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}
// impl can also apply to enums
impl Shape {
    fn area(&self) -> f64 {
        // calculate size
    }
}

// to check enum type
// way 1(need #[derive(PartialEq)])
animal == AnimalType::Dog
// wat 2
if let Shape::Rectangle { width, height } = shape2 {
    println!("rectangle with width {} and height {}", width, height)
}
// wat 3
match shape1 {
    Shape::Circle { radius, .. } => std::f64::consts::PI * radius.powi(2),
    Shape::Rectangle { width, height } => width * height,
}
```

# Collections
```rust
// simple structures can be create by tuples
let person = ("Rust", 18);
println!("name = {}, age = {}", person.0, person.1);
let (name, age) = person;

// List of items is described as Vector
let mut numbers: Vec<i32> = vec![0];
// most of list actions can be found in the Iterator 
println!("sum = {}", numbers.iter().sum::<i32>());
let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<i32>>();

// creating hashmap
let mut map = HashMap::new();
```

# Optionals
```rust
// safely unwrap
// 1. use match
// 2. use if let
// 3. use unwrap_or
```