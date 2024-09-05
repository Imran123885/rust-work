fn main() {
    let i = 13;
    let alpha = sigma {
        weight: 132,
        height: 60,
    };
    take_primitive(i);
    take_primitive(i); // For primitive data types, such as integers or booleans, since they implement the Copy trait, a copy is made in the new function, meaning the orignal function can still access the variable in its scope

    take_custom(alpha);
    take_custom(alpha); // For custom data types or other datatypes that do not have the Copy trait implemented (Strings, vecs, etc), you have to either implement the Copy trait manually or pass them as a reference to use them again
}


struct sigma {
    weight: i32,
    height: i32,
}

fn take_primitive(rand: i32) {
    println!("{:?}", rand);
}

fn take_custom(alpha: sigma) {
    println!("{:?} {:?}", alpha.height, alpha.weight);
}