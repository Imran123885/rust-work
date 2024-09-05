fn main() {
    let people = vec![
        person_info {
            age: 13,
            name: String::from("Imran Mirza"),
            color: Color::Green,
        },
        person_info {
            age: 9,
            name: String::from("Simga Alpha"),
            color: Color::Yellow,
        },
        person_info {
            age: 33,
            name: String::from("Ohio Skibidi"),
            color: Color::Red,
        },
    ];

    for person in people {
        if person.age <= 10 {
            println!("Name: {:?} | Age: {:?}", person.name, person.age);
            match person.color {
                Color::Green => println!("Favorite Color: Green"),
                Color::Yellow => println!("Favorite Color: Yellow"),
                Color::Red => println!("Favorite Color: Red"),
            }
        }
    }
}

enum Color {
    Green, 
    Yellow,
    Red,
}

struct person_info {
    age: i32,
    name: String,
    color: Color,
}