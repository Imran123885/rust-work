// NOTE: Use Tuples for up to 3 pieces of data / elements. If you have more, use a struct. 
fn main() {
    let (_x, _y) = create_tuple();
    if _y < 5 {
        println!("Y coord: {_y:?} is less than 5")
    } else if _y == 5 {
        println!("Y coord: {_y:?} is equal to 5")
    } else {
        println!("Y coord: {_y:?} is greater than 5")
    }
}

fn create_tuple() -> (i32, i32) {
    (5, 5)
}