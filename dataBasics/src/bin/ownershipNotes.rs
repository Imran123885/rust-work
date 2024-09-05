// OWNERSHIP in rust

// Programs HAVE to track memory. If they don't, that could lead to memory leaks

// In order to track/manage memory, rust uses and Ownership model.
// In this model, the owner is responsible for cleaning up memory. 
// In rust, the owner is just a function

// Memory can be moved or borrowed


// Example of how we can get cooked due to memory 
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright!"),
        Light::Dull => println!("dull!"),
    }
}


fn main() {
    println!("This file has notes on what ownership is in Rust!!");
    let dull_light = Light::Dull;
    display_light(&dull_light);
    display_light(&dull_light);
    println!("Address to dull_light var in memory: {:p}", &dull_light);
}

// Over here, the dull_light variable is moved from being owned by the main function to the display_light function, which then deletes the dull_light
// This means that we can not use the dull_light variable again.