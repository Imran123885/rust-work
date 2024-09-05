fn main() {
    let my_nums = vec![10, 20, 30, 40];
    for num in &my_nums { // Since you can transfer ownership to loops as well, you have to pass my_nums as a reference in order for you to access my_nums outside of the loop again in the main function
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("Length of my_nums: {:?}", my_nums.len());
}