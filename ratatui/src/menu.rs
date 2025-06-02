#[derive(Debug, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: f64,
}

impl Dish {
    pub fn new(name: &str, description: &str, price: f64) -> Self {
        Dish {
            name: name.to_string(),
            description: description.to_string(),
            price,
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

    pub fn add_dish(&mut self, dish: Dish) {
        self.dishes.push(dish);
    }
}
