fn main() {
    let i = 101;
    let size = i > 100;
    is_size(size);

}

fn is_size(boolean: bool) {
    match boolean {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}