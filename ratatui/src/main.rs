use kitchen::Kitchen;
use menu::Menu;
use orders::{Order, Orders};
use quotes::Quotes;
use std::io::Write;

mod kitchen;
mod menu;
mod orders;
mod quotes;

struct Main;

impl Main {
    fn run() {
        let dishes = vec![];
        let mut menu = Menu::new(dishes);
        let mut orders = Orders::new();
        let mut kitchen = Kitchen::new(orders);
        let quotes = Quotes::new();

        loop {
            Self::show_menu();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => {
                    menu.list_dishes();
                }
                "2" => {
                    // Add order (simplified)
                    println!("Enter table number:");
                    let mut table_input = String::new();
                    std::io::stdin().read_line(&mut table_input).unwrap();
                    let table: u32 = table_input.trim().parse().unwrap_or(0);

                    println!("Enter dish number:");
                    let mut dish_input = String::new();
                    std::io::stdin().read_line(&mut dish_input).unwrap();
                    let dish_idx: usize = dish_input.trim().parse().unwrap_or(0);

                    if let Some(dish) = menu.dishes.get(dish_idx) {
                        let order = Order::new(table, dish.clone());
                        orders.add_order(order);
                        println!("Order added!");
                    } else {
                        println!("Invalid dish number.");
                    }
                }
                "3" => {
                    orders.update_status(&mut orders);
                }
                "4" => {
                    kitchen.prepare_order(&mut orders);
                }
                "5" => {
                    kitchen.complete_order(&mut orders);
                }
                "6" => {
                    println!("{}", quotes.get_random_quote());
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
        println!("1. List Dishes");
        println!("2. Add Order");
        println!("3. Update Order Status");
        println!("4. Prepare Order");
        println!("5. Complete Order");
        println!("6. Show Random Quote");
        println!("0. Exit");
        print!("Select an option: ");
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    Main::run();
}
