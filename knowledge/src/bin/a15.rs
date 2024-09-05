fn main() {
    let tickets = vec![
        Ticket::Standard(50),
        Ticket::Backstage(200, String::from("Danish")),
        Ticket::Vip(200, "Zain".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Price for Standard Ticket: {:?}", price),
            Ticket::Backstage(price, name) => println!("Price for Backstage Ticket: {:?} | Name of Ticket Holder: {:?}", price, name),
            Ticket::Vip(price, name) => println!("Price for VIP Ticket: {:?} | Name of Ticket Holder: {:?}", price, name),
        }
    }
}

enum Ticket {
    Standard(i32),
    Backstage(i32, String),
    Vip(i32, String),
}

