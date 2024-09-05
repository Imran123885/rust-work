fn main() {
    let adult = Adult::new(32, "Danish");
    let child = Adult::new(13, "Danish");

    match adult {
        Ok(adult) => println!("This is the information about the adult: {:?}", adult),
        Err(e) => println!("Error Message: {:?}", e),
    }

    match child {
        Ok(adult) => println!("This is the information about the adult: {:?}", adult),
        Err(e) => println!("Error Message: {:?}", e),
    }
}

#[derive(Debug)]
struct Adult {
    Name: String, 
    Age: i32,
}

impl Adult {
    fn new(age: i32, name: &str) -> Result<Self, String> {
        if age >= 21 {
            return Ok(Self {
                Name: name.to_owned(),
                Age: age,
            });
        } else {
            Err(String::from("You are stupid as hell boss"))
        }
    }
}