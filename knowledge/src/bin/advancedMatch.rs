fn main() {
    let n = 3;
    match n {
        3 => println!("3"),
        // _ => println!("Not 3"), THIS IS A BAD WAY TO DO IT
        other => println!("{:?}", other),
    }

    let flat = Discount::Flat(3);

    match flat {
        Discount::Flat(amount) => println!("Flat Discount: {:?}", amount),
        Discount::Percent(amount) => println!("Percent Discount: {:?}", amount),
    }
    
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket { price: 50, event} => println!("event @ 50 {:?}", event),
        Ticket { price, ..} => println!("price = {:?}", price),
    }
}

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32
}