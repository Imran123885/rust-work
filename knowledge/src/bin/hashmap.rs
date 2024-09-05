use std::collections::HashMap;

fn main() {
    let mut lockers: HashMap<i32, Contents> = HashMap::new();
    lockers.insert(
        1,
        Contents {
        content: String::from("Stuff")
        },
    );
    lockers.insert(
        2,
        Contents {
        content: String::from("Shirt")
        },
    );
    lockers.insert(
        3,
        Contents {
        content: String::from("Legs")
        },
    );

    for (locker_number, item) in lockers.iter() {
        println!("Number: {:?} | Content: {:?}", locker_number, item);
    }
}

#[derive(Debug)]
struct Contents {
    content: String,
}