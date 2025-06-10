mod shipment;
mod shipment_manager;

use crate::shipment::{Package, Shipment, ShipmentStatus};
use crate::shipment_manager::ShipmentManager;
use std::io::{self, Write};
use std::process::exit;

fn print_help() {
    println!("Available commands:");
    println!("    add-shipment    - Add a new shipment with tracking ID and destination");
    println!("    add-package     - Add a package to an existing shipment");
    println!("    update-status   - Update the status of a shipment");
    println!("    view-shipment   - View details of a specific shipment");
    println!("    list-shipments  - List all shipments (optionally filter by status)");
    println!("    generate-report - Generate shipment/package reports");
    println!("    clear           - Clear the screen");
    println!("    help            - Show available commands");
    println!("    exit            - Exit the Cargo Tracker CLI");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to Cargo Tracker 1.0!");
    println!("Type 'help' to see a list of available commands.");

    let mut manager = ShipmentManager::new();

    loop {
        let command = read_input("> ").to_lowercase();

        match command.as_str() {
            "help" => print_help(),
            "add-shipment" => {
                // TODO: Implement add-shipment logic
                println!("(add-shipment not yet implemented)");
            }
            "add-package" => {
                // TODO: Implement add-package logic
                println!("(add-package not yet implemented)");
            }
            "update-status" => {
                // TODO: Implement update-status logic
                println!("(update-status not yet implemented)");
            }
            "view-shipment" => {
                // TODO: Implement view-shipment logic
                println!("(view-shipment not yet implemented)");
            }
            "list-shipments" => {
                // TODO: Implement list-shipments logic
                println!("(list-shipments not yet implemented)");
            }
            "generate-report" => {
                // TODO: Implement generate-report logic
                println!("(generate-report not yet implemented)");
            }
            "clear" => {
                // Clear screen (works on most Unix terminals)
                print!("\x1B[2J\x1B[1;1H");
            }
            "exit" => {
                println!("Goodbye!");
                exit(0);
            }
            "" => continue,
            _ => println!("Unknown command. Type 'help' to see available commands."),
        }
    }
}
