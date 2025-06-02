#[derive(Debug)]
pub struct Dish {
    pub name: String,
    pub ingredients: Vec<String>,
}
pub struct Menu {
    pub dishes: Vec<Dish>,
}

impl Menu {
    pub fn new(dishes: Vec<Dish>) -> Self {
        Menu { dishes }
    }
    pub fn list_dishes(&self) {
        for (i, dish) in self.dishes.iter().enumerate() {
            println!("{}. {}", i + 1, dish.name);
        }
    }
}
