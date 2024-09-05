fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    // match me.position {
    //     Position::Manager => println!("Position: Manager"),
    //     Position::Supervisor => println!("Position: Supervisor"),
    //     Position::Worker => println!("Position: Worker"),
    // } 

    // ABOVE IS TEDIOUS, use derice debug instead
    
    print_employee(me);
    print_employee(me);

    println!("{:?}", me.position);
    println!("{:?}", me);
}

#[derive(Debug, Clone, Copy)] // Way easier way of doing it
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)] 
struct Employee {
    position: Position,
    work_hours: i32,
}

fn print_employee(emp: Employee) { // NOT BORROWING
    println!("{:?}", emp);
}