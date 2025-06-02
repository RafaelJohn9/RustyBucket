use assert_cmd::Command;
use predicates::str::contains;

fn pixar_fail_joke(context: &str) -> String {
    format!(
        "\n🍿 Whoops! Looks like Remy tripped in the code pantry while: {}\n💬 Tip: Anyone can cook, but not everyone can assert output. Try again, Chef!",
        context
    )
}

#[test]
fn test_launching_kitchen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;
    let assert = cmd.write_stdin("\n6\n").assert();
    let output = String::from_utf8_lossy(&assert.get_output().stdout);

    // The menu header is always printed, so check for that
    if !output.contains("--- RustyBucket Restaurant ---") {
        return Err(pixar_fail_joke("starting the CLI welcome message").into());
    }
    Ok(())
}

#[test]
fn test_view_menu_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("1\n6\n")
        .assert()
        .stdout(contains("~~~Tonight's Menu~~~"))
        .stdout(contains(
            "0. Ratatouille Supreme - $18.50: A classic French Provençal stewed vegetable dish.",
        ))
        .stdout(contains(
            "1. Lightning Linguine - $15.00: Pasta tossed with zesty lemon and fresh herbs.",
        ))
        .stdout(contains(
            "2. Fromage Fantastique - $12.00: A selection of the finest French cheeses.",
        ));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("displaying tonight’s menu").into());
    }
    Ok(())
}

#[test]
fn test_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("banana\n6\n")
        .assert()
        .stdout(contains("🐒 Uh... Chef? “banana” is not a valid command."));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("handling invalid fruit commands").into());
    }
    Ok(())
}

#[test]
fn test_close_restaurant() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("6\n")
        .assert()
        .stdout(contains(
            "👋 Au revoir! May your soufflés rise and your bugs be shallow!",
        ))
        .stdout(contains(
            "💡 Pro tip: Don’t forget to clean the terminal before your next guest.",
        ));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("closing the restaurant").into());
    }
    Ok(())
}

#[test]
fn test_add_dish_to_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    // Simulate user input for adding a new dish:
    // 2 = Add Dish, then name, description, price, then 6 = Exit
    let result = cmd
        .write_stdin("2\nPizzeria\nOvergloried bread\n56.00\n6\n")
        .assert()
        .stdout(contains(
            "Dish 'Pizzeria' added to the menu! We hope no one orders it... again.",
        ));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("adding a new experimental dish").into());
    }
    Ok(())
}

#[test]
fn test_take_new_order() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    // 3 = Take Order, 5 = Table 5, 0 = Dish 0 (Ratatouille Supreme), 6 = Exit
    let result = cmd
        .write_stdin("3\n5\n0\n6\n")
        .assert()
        .stdout(contains("One 'Ratatouille Supreme' coming up!"));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("placing an order for the brave").into());
    }
    Ok(())
}

#[test]
fn test_advance_order_status() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    // Place an order first
    let result1 = cmd
        .write_stdin("3\n1\n1\n5\n1\n6\n")
        .assert()
        .stdout(contains("Order for table 1 is now 'Cooking'"));

    if let Err(_) = result1.try_success() {
        return Err(pixar_fail_joke("moving order from pending to cooking").into());
    }

    // Advance again to Served for table 1
    let mut cmd2 = Command::cargo_bin("ratatui")?;
    let result2 = cmd2
        .write_stdin("3\n1\n1\n5\n1\n5\n1\n6\n")
        .assert()
        .stdout(contains("Order for table 1 is now 'Served'"));

    if let Err(_) = result2.try_success() {
        return Err(pixar_fail_joke("serving the first dish").into());
    }

    // Try to advance again (should still say Served for table 1)
    let mut cmd3 = Command::cargo_bin("ratatui")?;
    let result3 = cmd3
        .write_stdin("3\n1\n1\n5\n1\n5\n1\n6\n")
        .assert()
        .stdout(contains("Order for table 1 is now 'Served'"));

    if let Err(_) = result3.try_success() {
        return Err(pixar_fail_joke("trying to re-cook a memory").into());
    }

    Ok(())
}

#[test]
fn test_overload_easter_egg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ratatui")?;

    let mut input = String::new();
    for i in 1..=6 {
        input += &format!("3\n{}\n0\n", i);
    }
    input += "4\n6\n"; // End the test gracefully

    let result = cmd
        .write_stdin(input)
        .assert()
        .stdout(contains("⚠️ Chef, we’re at MAXIMUM OVERLOAD!"))
        .stdout(contains(
            "Linguini has fainted. Colette is sharpening knives.",
        ))
        .stdout(contains(
            "Suggest: “Advance Order Status” or hide in the pantry.",
        ));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("triggering kitchen panic mode").into());
    }
    Ok(())
}
