fn main() {
    // let choice = get_choice("mainmenu");
    // println!("Choice: {:?}", choice);
    // match choice {
    //     Ok(value) => print_choice(&value),
    //     Err(value) => println!("Error has occured: {:?}", value),
    // }

    // let choice2 = get_choice("sigma");
    // println!("Choice: {:?}", choice2);
    // match choice2 {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(e) => println!("Error has occured: {:?}", e),
    // }
    
    let choice = pick_choice("engima");

    println!("Choice Value: {:?}", choice);
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Your options are: [mainmenu, start, quit]. Please try again with a valid option from the given list.".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    match choice {
        MenuChoice::MainMenu => println!("Menu Choice: Main Menu"),
        MenuChoice::Start => println!("Menu Choice: start"),
        MenuChoice::Quit => println!("Menu Choice: Quit"),
    }
}