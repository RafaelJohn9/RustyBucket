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

        let mut menu = Menu::new(dishes);
        let orders = Orders::new();
        let mut kitchen = Kitchen::new(orders);

        let mut output_buffer: Vec<String> = Vec::new();

        loop {
            // Clear terminal screen
            print!("\x1B[2J\x1B[1;1H");

            // Print buffered output before menu
            for line in &output_buffer {
                println!("{}", line);
            }
            output_buffer.clear();

            Self::show_menu();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => {
                    output_buffer.push("\t~~~Tonight's Menu~~~".to_string());
                    let dishes_output = menu
                        .dishes
                        .iter()
                        .enumerate()
                        .map(|(i, d)| {
                            format!("{}. {} - ${:.2}: {}", i, d.name, d.price, d.description)
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    output_buffer.push(dishes_output);
                    output_buffer.push("\n\t\tBon appétit! 🍽️".to_string());
                }
                "2" => {
                    // Prompt for dish details
                    output_buffer.push("Enter dish name:".to_string());
                    // Print buffered output and prompt before reading input
                    for line in &output_buffer {
                        println!("{}", line);
                    }
                    output_buffer.clear();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    let mut name_input = String::new();
                    std::io::stdin().read_line(&mut name_input).unwrap();
                    let name = name_input.trim().to_string();

                    // Prompt for description directly
                    println!("Enter dish description:");
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    let mut desc_input = String::new();
                    std::io::stdin().read_line(&mut desc_input).unwrap();
                    let description = desc_input.trim().to_string();

                    // Prompt for price directly
                    println!("Enter dish price:");
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    let mut price_input = String::new();
                    std::io::stdin().read_line(&mut price_input).unwrap();
                    let price: f64 = price_input.trim().parse().unwrap_or(0.0);

                    let new_dish = Dish::new(&name, &description, price);
                    kitchen
                        .current_orders
                        .add_order(Order::new(0, new_dish.clone()));
                    menu.add_dish(new_dish);
                    output_buffer.push(format!("Dish '{}' added to the menu! We hope no one orders it... again.", name));
                }
                "3" => {
                    // Check for maximum overload (5+ pending orders)
                    let pending_orders = kitchen
                        .current_orders
                        .orders
                        .iter()
                        .filter(|o| o.status == orders::OrderStatus::Pending)
                        .count();
                    if pending_orders >= 5 {
                        output_buffer.push("⚠️ Chef, we’re at MAXIMUM OVERLOAD!\n5+ orders are pending! This is not a drill!\nLinguini has fainted. Colette is sharpening knives.\nSuggest: “Advance Order Status” or hide in the pantry.".to_string());
                        for line in &output_buffer {
                            println!("{}", line);
                        }
                        output_buffer.clear();
                        std::io::stdout().flush().unwrap();
                        continue;
                    }

                    // Prompt for table number
                    output_buffer.push("Enter table number:".to_string());
                    // Print buffered output and prompt before reading input
                    for line in &output_buffer {
                        println!("{}", line);
                    }
                    output_buffer.clear();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    let mut table_input = String::new();
                    std::io::stdin().read_line(&mut table_input).unwrap();
                    let table: u32 = table_input.trim().parse().unwrap_or(0);

                    if kitchen.current_orders.get_order_for_table(table).is_some() {
                        output_buffer.push(format!("Table {} already has an order.", table));
                        continue;
                    }

                    // Show dishes for selection
                    let dishes_output = menu
                        .dishes
                        .iter()
                        .enumerate()
                        .map(|(i, d)| {
                            format!("{}. {} - ${:.2}: {}", i, d.name, d.price, d.description)
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    println!("{}", dishes_output);

                    print!("\nEnter dish number: ");
                    std::io::stdout().flush().unwrap();
                    let mut dish_input = String::new();
                    std::io::stdin().read_line(&mut dish_input).unwrap();
                    let dish_idx: usize = dish_input.trim().parse().unwrap_or(0);

                    if let Some(dish) = menu.dishes.get(dish_idx) {
                        let order = Order::new(table, dish.clone());
                        kitchen.current_orders.add_order(order);

                        // Humor if dish is not in default_dishes
                        let is_default = {
                            let mut defaults = Vec::new();
                            default_dishes(&mut defaults);
                            defaults.iter().any(|d| d.name == dish.name)
                        };
                        if !is_default {
                            output_buffer.push(format!(
                                "🧾 Table {} has ordered “{}”.\n😬 Bold choice. We admire their courage.",
                                table, dish.name
                            ));
                        } else {
                            output_buffer.push(format!("One '{}' coming up!", dish.name));
                        }
                    } else {
                        output_buffer.push("Invalid dish number.".to_string());
                    }
                }
                "4" => {
                    let mut orders_output = String::new();
                    for order in &kitchen.current_orders.orders {
                        orders_output.push_str(&format!(
                            "Table {}: {} - Status: {:?}\n",
                            order.table, order.dish.name, order.status
                        ));
                    }
                    if orders_output.is_empty() {
                        output_buffer.push("No orders yet.".to_string());
                    } else {
                        output_buffer.push(orders_output);
                    }
                }
                "5" => {
                    output_buffer.push("Enter table number:".to_string());
                    // Print buffered output and prompt before reading input
                    for line in &output_buffer {
                        println!("{}", line);
                    }
                    output_buffer.clear();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    let mut table_input = String::new();
                    std::io::stdin().read_line(&mut table_input).unwrap();
                    let table_id: u32 = table_input.trim().parse().unwrap_or(0);
                    kitchen.current_orders.update_status(table_id);

                    if let Some(order) = kitchen.current_orders.get_order_for_table(table_id) {
                        output_buffer.push(format!(
                            "Order for table {} is now '{:?}'.",
                            table_id, order.status
                        ));
                    } else {
                        output_buffer.push(format!("No order found for table {}.", table_id));
                    }
                }
                "6" => {
                    println!("👋 Au revoir! May your soufflés rise and your bugs be shallow!");
                    println!("💡 Pro tip: Don’t forget to clean the terminal before your next guest.");
                    break;
                }
                invalid => output_buffer.push(format!(
                    "🐒 Uh... Chef? “{}” is not a valid command. This isn’t a fruit stand.\nTry typing a number from the list, like a responsible rodent.\n~",
                    invalid
                )),
            }
        }
    }
    fn show_menu() {
        println!("\n- - - RustyBucket Restaurant - - -\n");
        println!("~ {} ~", Quotes::new().get_random_quote());
        println!("1. View Menu");
        println!("2. Add Dish to Menu");
        println!("3. Take New Order");
        println!("4. View all Orders");
        println!("5. Advance Order Status");
        println!("6. Close Restaurant (Exit)");
        print!("Select an option: ");
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    Main::run();
}
