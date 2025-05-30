fn main() {
    // Allocate a String on the heap
    let s = String::from("Hello, Rust");

    // Pass an immutable reference (&s) to the function — this is borrowing
    print_string(&s);

    // The original String is still valid here because it was borrowed, not moved
    println!("Original string is still valid: {}", s);
}

// Function that takes an immutable reference to a String
fn print_string(data: &String) {
    // Use the borrowed data — no ownership taken
    println!("Borrowed: {}", data);
}