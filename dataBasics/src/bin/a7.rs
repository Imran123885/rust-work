fn main() {
    choose_color(Color::Green)
}

enum Color {
    Red,
    Green,
    Purple,
}

fn choose_color(cc: Color) {
    match cc {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Purple => println!("Purple"),
    }
}