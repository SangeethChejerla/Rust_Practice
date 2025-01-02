fn increment(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut num = 5;
    increment(&mut num);
    println!("num after increment: {}", num); // Output: 6

    let ptr = &mut num as *mut i32;
    unsafe {
        *ptr += 1;
    }
    println!("num after unsafe increment: {}", num); // Output: 7
}
