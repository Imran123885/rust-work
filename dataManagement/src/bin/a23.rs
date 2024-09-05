fn main() {

}

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

