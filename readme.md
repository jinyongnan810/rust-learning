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
