fn main() {
    let student_locker = Locker::new("Imran".to_owned(), Some(34));

    student_locker.info();
}

struct Locker {
    name: String,
    locker: Option<i32>,
}

impl Locker {
    fn new(name: String, lock_num: Option<i32>) -> Self {
        Self {
            name: name,
            locker: lock_num,
        }
    }

    fn info(&self) {
        println!("Name of Student: {:?}", self.name);
        match self.locker {
            Some(lock_num) => println!("Locker Number: {:?}", lock_num),
            None => println!("No locker number provided"),
        }
    }
}