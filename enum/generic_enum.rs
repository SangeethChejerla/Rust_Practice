#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

fn main() {
    let number: Maybe<i32> = Maybe::Just(7);
    let nothing: Maybe<String> = Maybe::Nothing;
    println!("Number: {:?}", number);
    println!("Nothing: {:?}", nothing);
}
