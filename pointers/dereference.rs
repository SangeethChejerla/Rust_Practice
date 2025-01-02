fn main() {
    let x = 5;                         // 1. Declare an immutable variable x with the value 5
    let ptr = &x as *const i32;       // 2. Create a raw pointer `ptr` pointing to `x`
    
    unsafe {                         // 3. Start an `unsafe` block
        println!("Dereferenced value: {}", *ptr); // 4. Dereference `ptr` and print the value
    }                                  // 5. End the `unsafe` block
}
