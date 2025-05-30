fn main() {
    let s = String::from("Hello, Rust");
    print_string(&s);
    println!("Original string is still valid: {}", s);
}

fn print_string(data: &String) {
    println!("Borrowed: {}", data);
}