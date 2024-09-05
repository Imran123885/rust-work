struct GroceryItem {
    stock: i32,
    price: f64,
}
fn main() {
    let cereal = GroceryItem {
        stock: 30, 
        price: 4.50,
    };
    println!("Stock of Cereal: {:?} | Price of Cereal: {:?}", cereal.stock, cereal.price)
}