// String: owned string
// &str: borrowed string slice (pass by reference)
fn main() {
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);
}


fn print_it(data: &str) {
    println!("{:?}", data);
}

