fn main() {
    let box_int = Box::new(100);
    println!("Value in box: {}", *box_int); // Output: Value in box: 100

    // Move the box
    let another_box = box_int;
    // println!("Value in box: {}", *box_int); // Error: value borrowed here after move
    println!("Value in another_box: {}", *another_box); // Output: Value in another_box: 100
}
