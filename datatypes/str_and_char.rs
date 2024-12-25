fn main() {
    // String literal (type &str, immutable)
    let greeting: &str = "Hello, Rust!"; 
    println!("{}", greeting); // Output: Hello, Rust!

    // String object (type String, mutable and growable)
    let mut message: String = String::from("This is a String.");
    message.push_str(" It can be modified.");
    println!("{}", message); // Output: This is a String. It can be modified.
    
    // Character literal (type char)
    let first_letter: char = 'A';
    println!("{}", first_letter); // Output: A

    let emoji: char = '😀'; 
    println!("{}", emoji); // Output: 😀
}
