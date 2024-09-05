use std::io;

fn main() {
    loop {
        let inp = get_input();
        match inp {
            Ok(words) => {
                match power_states::new(&words) {
                    Some(power_states::Off) | Some(power_states::Shutdown) => {
                        print_power_states(power_states::new(&words).unwrap());
                        break
                    },
                    Some(state) => print_power_states(state),
                    None => {
                        println!("Please choose one of the selected options\n");
                        print_power_states(power_states::Options);
                    },
                }
            },
            Err(e) => println!("Error: {:?}", e),
    
        }
    }

}

enum power_states {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    Options,
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    println!("Please input a power state for the computer (input Options for all the options): ");
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}


impl power_states {
    fn new(state: &str) -> Option<Self> {
        let state: String = state.trim().to_lowercase();
        // String -> &str
        match state.as_str() {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            "options" => Some(Self::Options),
            _ => None,
        }
    
    }
}

fn print_power_states(state: power_states) {
    use power_states::*;
    
    match state {
        Off => println!("power off..."),
        Sleep => println!("Computer sleeping.."),
        Reboot => println!("Coputer rebooting"),
        Shutdown => println!("Shutting down..."),
        Hibernate => println!("Hibernating"),
        Options => println!("Options: \n 1. Off \n 2. Sleep \n 3. Reboot \n 4. Shutdown \n 5. Hibernate"),
    }
}