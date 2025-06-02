use kitchen::Kitchen;
use menu::Dish;
use menu::Menu;
use orders::{Order, Orders};
use quotes::Quotes;
use std::io::Write;

mod kitchen;
mod menu;
mod orders;
mod quotes;

fn default_dishes(dishes: &mut Vec<Dish>) {
    dishes.clear();
    dishes.push(Dish::new(
        "Ratatouille Supreme",
        "A classic French Provençal stewed vegetable dish.",
        18.50,
    ));
    dishes.push(Dish::new(
        "Lightning Linguine",
        "Pasta tossed with zesty lemon and fresh herbs.",
        15.00,
    ));
    dishes.push(Dish::new(
        "Fromage Fantastique",
        "A selection of the finest French cheeses.",
        12.00,
    ));
    dishes.push(Dish::new(
        "Parisian Poulet",
        "Roasted chicken with herbes de Provence.",
        20.00,
    ));
    dishes.push(Dish::new(
        "Crème Brûlée",
        "Rich vanilla custard with a caramelized sugar top.",
        8.50,
    ));
}

struct Main;

impl Main {
    fn run() {
        let mut dishes = vec![];
        default_dishes(&mut dishes);

        let menu = Menu::new(dishes);
        let orders = Orders::new();
        let mut kitchen = Kitchen::new(orders);

        // Clear terminal screen
        print!("\x1B[2J\x1B[1;1H");
        loop {
            Self::show_menu();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => {
                    menu.list_dishes();
                }
                "2" => {
                    println!("Enter table number:");
                    let mut table_input = String::new();
                    std::io::stdin().read_line(&mut table_input).unwrap();
                    let table: u32 = table_input.trim().parse().unwrap_or(0);

                    if kitchen.current_orders.get_order_for_table(table).is_some() {
                        println!("Table {} already has an order.", table);
                        continue;
                    }

                    menu.list_dishes();
                    println!("\n\nEnter dish number:");
                    let mut dish_input = String::new();
                    std::io::stdin().read_line(&mut dish_input).unwrap();
                    let dish_idx: usize = dish_input.trim().parse().unwrap_or(0);

                    if let Some(dish) = menu.dishes.get(dish_idx) {
                        let order = Order::new(table, dish.clone());
                        kitchen.current_orders.add_order(order);
                        println!("One   '{}'   coming up!", dish.name);
                    } else {
                        println!("Invalid dish number.");
                    }
                }
                "3" => {
                    println!("Enter table number:");
                    let mut table_input = String::new();
                    std::io::stdin().read_line(&mut table_input).unwrap();
                    let table_id: u32 = table_input.trim().parse().unwrap_or(0);
                    kitchen.current_orders.update_status(table_id);

                    if let Some(order) = kitchen.current_orders.get_order_for_table(table_id) {
                        println!("Order for table {} is now '{:?}'.", table_id, order.status);
                    } else {
                        println!("No order found for table {}.", table_id);
                    }
                }
                "4" => {
                    kitchen.show_all_orders();
                }
                "0" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid option."),
            }
        }
    }

    fn show_menu() {
        println!("\n--- RustyBucket Restaurant ---");
        println!("~ {} ~", Quotes::new().get_random_quote());
        println!("1. List Dishes");
        println!("2. Add Order");
        println!("3. Update Order Status");
        println!("4. View all Orders");
        println!("0. Exit");
        print!("Select an option: ");
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    Main::run();
}
