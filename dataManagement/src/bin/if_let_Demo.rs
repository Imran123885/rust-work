fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user: {:?}", user),
        None => println!("No User"),
    }

    if let Some(user) = maybe_user {
        println!("user: {:?}", user);
    } else {
        println!("No user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("It's Red!");
    } else {
        println!("IT is not red");
    }
}

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}