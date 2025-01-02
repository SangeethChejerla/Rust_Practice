


fn main() {
    let x: i32 = 5;
    let _y: i32; // Intentionally not used and no warning.
    
    assert_eq!(x, 5);
    println!("Success! x is: {}", x); // Print x's value
}
