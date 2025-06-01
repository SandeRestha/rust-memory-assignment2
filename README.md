# Rust Memory Management – Assignment 2

This project demonstrates how Rust handles memory management using its unique ownership model. It illustrates key concepts such as heap allocation, borrowing, and compile-time guarantees against memory safety issues.

## Project Structure

- `src/main.rs` — Main Rust source file containing the demonstration code
- `Cargo.toml` — Project configuration file

## Code Summary

```rust
fn main() {
    let s = String::from("Hello, Rust");
    print_string(&s);
    println!("Original string is still valid: {}", s);
}

fn print_string(data: &String) {
    println!("Borrowed: {}", data);
}

## File
- `main.rs`: Contains the main program logic

## How to Run

Make sure you have Rust and Cargo installed. Then:

```bash
cargo run
