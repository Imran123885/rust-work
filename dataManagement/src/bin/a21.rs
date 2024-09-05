fn main() {
    let username = "sam";
    let user = find_user(username).map(|id| User {user_id: id, name: username.to_owned()});

    match user {
        Some(u) => println!("User: {:?}", u),
        None => println!("User not found"),
    }
}


fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}


#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

