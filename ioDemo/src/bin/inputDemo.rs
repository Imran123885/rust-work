use std::io;

fn get_input() -> io::Result<String> { // No need to add Err result tyupe
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn main() {
    let mut all_input = Vec::new();
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    for input in all_input {
        println!("Original: {:?} | Capitilized: {:?}", input, input.to_uppercase());
    }
}
