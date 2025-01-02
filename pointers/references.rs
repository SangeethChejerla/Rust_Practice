fn main() {
    let x = 5;          // x is immutable
    let y = &x;         // y is a reference to x
    println!("y: {}", y); // Output: y: 5

    let mut z = 10;     // z is mutable
    let a = &mut z;     // a is a mutable reference to z
    *a += 5;            // Modify through reference
    println!("z: {}", z); // Output: z: 15
}
