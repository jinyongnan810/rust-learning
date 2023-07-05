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
