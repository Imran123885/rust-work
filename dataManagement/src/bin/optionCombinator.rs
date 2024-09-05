fn main() {
    let a: Option<i32> = None;
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
    let a_filtered = a.filter(|num| num == &1); // Filtered ALWAYS borrows the number
    let a_or_else = a.or_else(|| Some(5)); // If a is None, then it returns a value instead of returning nothing
    let unwrapped = a.unwrap_or_else(|| 0); // Takes data out of Some instead of what or_else does
    // println!("{:?}", a_or_else);
}