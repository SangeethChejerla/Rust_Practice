enum Color {
    Red(f64), // Associated with a float for brightness
    Green,
    Blue(bool), // Associated with a boolean for isLight
}

fn print_color(color: Color) {
    match color {
        Color::Red(brightness) => println!("Red with brightness: {}", brightness),
        Color::Green => println!("Just green"),
        Color::Blue(is_light) => println!("Blue, is it light? {}", is_light),
    }
}

fn main() {
    let red = Color::Red(0.8);
    let green = Color::Green;
    let blue = Color::Blue(true);
    print_color(red);
    print_color(green);
    print_color(blue);
}
