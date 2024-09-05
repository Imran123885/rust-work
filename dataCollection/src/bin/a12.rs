fn main() {
    let my_box = Box::create_box();

    my_box.characteristics();

    let my_dims = Dimensions {
        width: 313.3,
        length: 67.2,
        depth: 12.1,
    };

    let my_creative_box = Box {
        color: Color::White,
        dimensions: my_dims,
        weight: 68,
    };

    Box::characteristics(&my_creative_box); 
}

enum Color {
    Brown,
    White,
}

struct Dimensions {
    width: f64,
    length: f64,
    depth: f64,
}
struct Box {
    color: Color,
    dimensions: Dimensions,
    weight: i32,
}

impl Box {
    fn create_box() -> Self {
        let dimensions = Dimensions {
            width: 56.3,
            length: 45.2,
            depth: 34.1,
        };

        Self {
            color: Color::Brown, 
            dimensions:  dimensions,
            weight: 64,
        }
    }

    fn characteristics(&self) {
        match self.color {
            Color::Brown => println!("Color: Brown"),
            Color::White => println!("Color: White"),
        }

        println!("Width: {:?} | Length: {:?} | Depth: {:?}", self.dimensions.width, self.dimensions.length, self.dimensions.depth);
        println!("Weight: {:?}", self.weight);
    }
}