fn main() {
    let numbers = vec![1,2,3];
    match numbers.is_empty() {
        true => println!("Vector does NOT contain numbers"),
        false => println!("Vector does contain numbers"),
    }
}