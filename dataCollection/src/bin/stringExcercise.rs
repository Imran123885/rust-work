fn main() {
    let receipt = vec![
        LineItem{ name: "cereal".to_owned(), count: 5},
        LineItem{ name: String::from("fruit"), count: 1},
    ];

    for item in receipt {
        print_name(&item.name);
        LineItem::info(&item);
    }
}

struct LineItem {
    name: String,
    count: i32,
}

impl LineItem {
    fn info(&self) {
        println!("Name: {:?}", self.name);
        println!("Count: {:?}", self.count);
    }
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}