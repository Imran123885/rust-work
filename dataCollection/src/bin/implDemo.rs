fn main() {
    let hot = Temperature { degrees_f: 100.0 };
    // Temperature::show_temp(&hot); ALTERNATE WAY TO SHOW THIS
    hot.show_temp();
    
    let cold = Temperature::freezing();
    cold.show_temp();
}

struct Temperature {
    degrees_f: f64,
}

// fn show_temp(temp: Temperature) {
//     println!("{:?} degrees F", temp.degrees_f);
// }

// THE ABOVE SHOWS THE show_temp FUNCTION OUTSIDE OF THE IMPL BLOCK, INDEPENDENT FROM THE TEMPERATURE STRUCT

impl Temperature { // ALLOWS FOR YOU TO IMPLEMENT A FUNCTION DIRECTLY WITHIN A STRUCT
    fn freezing() -> Self { // Self with a captital S indicates that a new instance of the object has to be created 
        Self {
            degrees_f: 32.0
        }
    }

    fn show_temp(&self) { // self with a lowercase s indicates that an instance of the object already exists and is being passed into the function as an argument
        println!("{:?} degrees F", self.degrees_f); 
    }
}