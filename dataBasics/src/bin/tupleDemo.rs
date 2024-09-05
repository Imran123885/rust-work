fn main() {
    let coord = (2, 3);
    println!("x-coordinate: {:?} | y-coordinate: {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("x-coordinate: {:?} | y-coordinate: {:?}", x, y);

    let user_info = ("Emma", 20); // Multiple data types can be in a tuple
    println!("Without destructuring:\nUser's name: {:?} | User age: {:?}", user_info.0, user_info.1);
    // Better way to get tuple data is by destructuring
    let (name, age) = ("Emma", 20);
    println!("With destructuring:\nUser's name: {:?} | User age: {:?}", name, age);

}