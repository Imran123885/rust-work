/// THIS IS A DOCUMENTATION COMMENT
#[derive(Debug)]
enum Color {
    Sigma, 
    Ohio,
}

/// I WILL BE THE BEST AT RUST INSHALLAH
struct Mail {
    address: String,
}
/// Documenation is him
fn main() {
    println!("DOCUMENTATION IS COOL");

    let color1 = Color::Sigma;
    let color2 = Color::Ohio;

    let mail = Mail { address: String::from("Engima") };

    println!("{:?} {:?} {:?}", color1, color2, mail.address);
}