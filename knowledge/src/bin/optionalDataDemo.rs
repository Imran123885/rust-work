fn main() {
    sigma();

    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some("IMMUJAANIHAI".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1 Answer: {:?}", ans),
        None => println!("User hates us :("),
    }

    match response.q2 {
        Some(ans) => println!("q2 Answer: {:?}", ans),
        None => println!("User hates us :("),
    }

    match response.q3 {
        Some(ans) => println!("q3 Answer: {:?}", ans),
        None => println!("User hates us :("),
    }
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}


fn sigma() {
    println!("Option can either be: \n 1. None \n 2. Some");
    println!("Declare options with Option<type>");
}