use std::io;

fn main() {
    // Create Bills structure
    let mut bills = Bills::new();


    loop {
        // Display menu
        MainMenu::show();
        // make choice
        let input = get_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()) {
            Some(option) => option.action(&mut bills),
            None => return,
        }
    }
}

#[derive(Debug)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }

    fn remove(&mut self, input: &str) {
        self.inner.retain(|bill| bill.name != input);
    }

    fn update(&mut self, name: &str, amount: f64, new_name: &str) -> bool {
        let value_to_update = self.inner.iter().position(|bill| bill.name == name);
        match value_to_update {
            Some(index) => {
                self.inner[index] = Bill { name: new_name.to_owned(), amount };
                true
            },
            None => { 
                println!("There is no bill with that name.");
                false
            },
        }
    }
}

mod menu {
    use crate::*;

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        let bill = Bill {
            name,
            amount,
        };

        bills.add(bill);
        println!("Bill was added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() { // get_all() returns vector of referenecs of bills
            println!("Bill: {:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        view_bills(bills);
        println!("Input the name of the bill you want to get rid of:\n");
        let input: String = match get_input() {
            Some(inp) => inp,
            None => return,
        };
        let before_length = bills.inner.len();
        bills.remove(input.trim());
        
        if bills.inner.len() == before_length {
            println!("A bill with that name does not exist. No bills have been removed.");
        } else {
            println!("Bill of name {:?} has been removed", input.trim());
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        println!("Input the name of the bill you want to update:\n");
        let input: String = match get_input() {
            Some(inp) => inp,
            None => return,
        };
        println!("Input the updated name of the bill:\n");
        let new_name: String = match get_input() {
            Some(inp) => inp,
            None => return,
        };
        println!("Input the updated amount of the bill:\n");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        let bill_update_status = bills.update(input.trim(), amount, new_name.trim());
        if bill_update_status {
            println!("Your bill: {:?} has been updated. \n New Name: {:?} \n New Amount: {:?}", input.trim(), new_name.trim(), amount);
        } else {
            println!("No bills have been updated");
        }
    }
}

enum MainMenu {
    Add,
    View,
    Remove,
    Update,
    Invalid,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "add" | "1" => Some(Self::Add),
            "view" | "2" => Some(Self::View),
            "remove" | "3" => Some(Self::Remove),
            "update" | "4" => Some(Self::Update),
            "" | "exit" => None,
            _ => Some(Self::Invalid),
        }
    }
    fn show() {
        println!("");
        println!("\n-- Bill Manager --");
        println!("1. Add Bills");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Update Bills\n");
        println!("Enter nothing or exit to quit\n");
        println!("Enter Selection: ");
    }

    fn action(&self, bills: &mut Bills) {
        use crate::menu;
        match self {
            Self::Add => menu::add_bill(bills),
            Self::View => menu::view_bills(bills),
            Self::Remove => menu::remove_bill(bills),
            Self::Update => menu::update_bill(bills),
            Self::Invalid => println!("Please enter a valid option."),
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again!");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount: ");
    loop {
        let input = match get_input() {
            Some(inp) => inp,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse(); // .parse() tries to turn item into correct data type
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Try again, please enter a number"),
        }
    }
}