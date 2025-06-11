mod shipment;
mod shipment_manager;

use crate::shipment::{Package, ShipmentStatus};
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
                // Create a new shipment
                let tracking_id = read_input("Please enter the Tracking ID: ");
                if manager.get_shipment(&tracking_id).is_some() {
                    println!(
                        "Error: Shipment with tracking ID '{}' already exists.",
                        tracking_id
                    );
                    continue;
                }
                let destination = read_input("Please enter the destination: ");
                let status = ShipmentStatus::Pending;
                let time_of_departure = Some(chrono::Utc::now());
                let shipment_id = if tracking_id.is_empty() {
                    None
                } else {
                    Some(tracking_id.clone())
                };
                let shipment =
                    manager.create_shipment(status, destination, time_of_departure, shipment_id);
                println!("Shipment Created\n\n");

                // Let's add packages to the shipment
                println!("Let's add packages. Type 'q' to quit");
                let mut count = 0;
                let mut package_num = 1;
                loop {
                    let prompt = format!("Enter package #{} description: ", package_num);
                    let description = read_input(&prompt);
                    if description.trim().eq_ignore_ascii_case("q") {
                        break;
                    }
                    let package = Package::new(description);
                    shipment.add_package(package);
                    count += 1;
                    package_num += 1;
                }
                let tracking = &shipment.tracking_id;
                println!(
                    "{} package{} added to shipment '{}'.",
                    count,
                    if count == 1 { "" } else { "s" },
                    tracking
                );
            }
            "add-package" => {
                // Add a package to an existing shipment
                let tracking_id = read_input("Enter the Tracking ID of the shipment: ");
                if tracking_id.is_empty() {
                    println!("Tracking ID cannot be empty.");
                    return;
                }
                match manager.get_shipment(&tracking_id) {
                    Some(shipment) => {
                        let description = read_input("Enter package description: ");
                        if description.trim().is_empty() {
                            println!("Package description cannot be empty.");
                            return;
                        }
                        let package = Package::new(description);
                        shipment.add_package(package);
                        println!("Package added to shipment '{}'.", shipment.tracking_id);
                    }
                    None => {
                        println!("Shipment with Tracking ID '{}' not found.", tracking_id);
                    }
                }
            }
            "update-status" => {
                let tracking_id = read_input("Please enter the Tracking ID: ");
                match manager.get_shipment(&tracking_id) {
                    Some(shipment) => {
                        let status_input =
                            read_input("Enter new status (Pending, InTransit, Delivered, Lost): ");

                        let new_status = match status_input.trim().to_lowercase().as_str() {
                            "pending" => ShipmentStatus::Pending,
                            "intransit" => ShipmentStatus::InTransit,
                            "delivered" => {
                                shipment.time_of_arrival = Some(chrono::Utc::now());
                                ShipmentStatus::Delivered
                            }
                            "lost" => ShipmentStatus::Lost,
                            _ => {
                                println!(
                                    "Error: Invalid status. Valid options are: Pending, InTransit, Delivered, Lost."
                                );
                                return;
                            }
                        };
                        shipment.status = new_status.clone();
                        println!(
                            "Shipment '{}' status updated to {:?}.",
                            shipment.tracking_id, new_status
                        );
                    }
                    None => {
                        println!("Shipment with Tracking ID '{}' not found.", tracking_id);
                    }
                }
            }
            "view-shipment" => {
                let tracking_id = read_input("Please enter the Tracking ID: ");
                match manager.get_shipment(&tracking_id) {
                    Some(shipment) => {
                        println!("Tracking ID: {}", shipment.tracking_id);
                        println!("Destination: {}", shipment.destination);
                        println!("Status: {:?}", shipment.status);
                        println!("Packages:");
                        for package in &shipment.packages {
                            println!("({})   - {}", package.id, package.description);
                        }
                        println!();
                    }
                    None => {
                        println!("Shipment with Tracking ID '{}' not found.", tracking_id);
                    }
                }
            }
            "list-shipments" => {
                // List all shipments, optionally filter by status
                let filter = read_input(
                    "Filter by status; Pending, InTransit, Delivered, Lost: (leave empty for all): ",
                );

                let filter_trimmed = filter.trim().to_lowercase();
                let status_filter = match filter_trimmed.as_str() {
                    "" => None,
                    "pending" => Some(ShipmentStatus::Pending),
                    "intransit" => Some(ShipmentStatus::InTransit),
                    "delivered" => Some(ShipmentStatus::Delivered),
                    "lost" => Some(ShipmentStatus::Lost),
                    _ => {
                        println!("No shipments found for status '{}'.", filter);
                        None
                    }
                };
                let filtered_shipments = manager.list_shipments(status_filter);

                if filtered_shipments.is_empty() {
                    println!("No shipments found.");
                } else {
                    for shipment in filtered_shipments {
                        println!(
                            "Tracking ID: {} | Destination: {} | Status: {:?} | Packages: {} | Departure: {} | Arrival: {}",
                            shipment.tracking_id,
                            shipment.destination,
                            shipment.status,
                            shipment.packages.len(),
                            shipment
                                .time_of_departure
                                .map(|t| t.to_rfc3339())
                                .unwrap_or_else(|| "N/A".to_string()),
                            shipment
                                .time_of_arrival
                                .map(|t| t.to_rfc3339())
                                .unwrap_or_else(|| "N/A".to_string()),
                        );
                    }
                }
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
            _ => println!("'{}' is not a valid command.", command),
        }
    }
}
