enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let result: Result<i32, String> = Result::Ok(42);
    match result {
        Result::Ok(value) => println!("Success! The value is: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }
}
