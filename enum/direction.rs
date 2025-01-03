#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;
    println!("Direction is: {:?}", direction); // This uses the Debug trait to format output
}
