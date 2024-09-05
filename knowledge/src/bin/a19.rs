use std::collections::HashMap;

fn main() {
    let mut furniture = HashMap::new();

    furniture.insert(
        "Chairs", 
        5
    );
    furniture.insert(
        "Beds", 
        3
    );
    furniture.insert(
        "Tables", 
        2
    );
    furniture.insert(
        "Couches", 
        0
    );

    let mut furniture_num = 0;

    for (item, count) in furniture.iter() {
        furniture_num = furniture_num + count;
        let stock = if count == &0 {
            String::from("out of stock")
        } else {
            format!("{:?}", count)
        };
        println!("item: {:?} | stock: {:?}", item, stock);
    }

    println!("Total number of furniture in the store: {:?}", furniture_num);
}