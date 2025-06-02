#[derive(Debug, Clone)]
pub struct Dish {
    pub name: String,
    pub ingredients: Vec<String>,
}

impl Dish {
    pub fn new(name: &str, description: &str, price: f64) -> Self {
        Dish {
            name: name.to_string(),
            ingredients: vec![description.to_string(), format!("Price: ${:.2}", price)],
        }
    }
}

pub struct Menu {
    pub dishes: Vec<Dish>,
}

impl Menu {
    pub fn new(dishes: Vec<Dish>) -> Self {
        Menu { dishes }
    }
    pub fn list_dishes(&self) {
        print!("\n\n ~~ RustyBucket Restaurant Menu ~~\n");
        for (i, dish) in self.dishes.iter().enumerate() {
            println!("{}. {}", i + 1, dish.name);
        }
        print!("\n\n");
    }
}
