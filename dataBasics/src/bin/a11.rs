fn main() {
    let item = groceryItem {
        quantity: 7,
        id_number: 124,
    };

    display_quantity(&item);
    display_id(&item);

}

struct groceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(item: &groceryItem) {
    println!("Quantity of this grocery: {:?}", item.quantity); 
}

fn display_id(item: &groceryItem) {
    println!("ID of this grocery: {:?}", item.id_number); 
}