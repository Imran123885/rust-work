fn main() {
    let my_numbers = vec![1, 2, 3];

    let mut my_nums = Vec::new();
    my_nums.push(1);
    my_nums.push(2);
    my_nums.push(3);
    my_nums.pop();
    println!("Length of my_nums: {:?}", my_nums.len());

    let _two = my_nums[1];

    vec_items(my_nums);
    vec_items(my_numbers);
}

fn vec_items(vector: Vec<i32>) {
    for item in vector {
        println!("Item: {:?}", item);
    }
}
// Alternate ways to create a vector data structure in Rust

