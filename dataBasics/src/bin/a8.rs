fn main() {
    // drink_info(Flavor::Mango);
    // drink_info(Flavor::Chocolate);
    // drink_info(Flavor::Cucumber);
    let my_drink = Drink {
        flavor: Flavor::Mango,
        fluid_oz: 75.5,
    };
    let my_drink2 = Drink {
        flavor: Flavor::Chocolate,
        fluid_oz: 69.0,
    };
    let my_drink3 = Drink {
        flavor: Flavor::Cucumber,
        fluid_oz: 105.4,
    };

    print_drink(my_drink);
    print_drink(my_drink2);
    print_drink(my_drink3);


}

enum Flavor {
    Mango,
    Chocolate,
    Cucumber,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Mango => println!("Flavor: Mango"),
        Flavor::Chocolate => println!("Flavor: Chocolate"),
        Flavor::Cucumber => println!("Flavor: Cucumber"),
    }
    println!("Fluid Ounce Info: {:?}", drink.fluid_oz);
}

// struct flav_fl_info<'a> {
//     flav: &'a str,
//     fl: i32,
// }

// fn drink_info(drink: Flavor) {
//     match drink {
//         Flavor::Mango => {
//             let mango_drink = flav_fl_info {
//                 flav: "Mango",
//                 fl: 32,
//             };
//             println!("Flavor: {:?} | fl information: {:?}", mango_drink.flav, mango_drink.fl);
//         },
//         Flavor::Chocolate => {
//             let choc_drink = flav_fl_info {
//                 flav: "Chocolate",
//                 fl: 48,
//             };
//             println!("Flavor: {:?} | fl information: {:?}", choc_drink.flav, choc_drink.fl);
//         },
//         Flavor::Cucumber => {
//             let cucum_drink = flav_fl_info {
//                 flav: "Cucumber",
//                 fl: 64,
//             };
//             println!("Flavor: {:?} | fl information: {:?}", cucum_drink.flav, cucum_drink.fl);
//         },
//     }
// }