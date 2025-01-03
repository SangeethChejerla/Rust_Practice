use std::fmt;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

fn main() {
    let direction = Direction::North;
    println!("Direction is: {:?}", direction); // Now uses the Display trait for formatting
}
