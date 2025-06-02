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
    println!("🔬 Running Test: Launching Kitchen — Opening scene in the Terminal of Taste™ 🐀");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let assert = cmd.write_stdin("\n").assert();
    let output = String::from_utf8_lossy(&assert.get_output().stdout);

    if !output.contains("🎩 Bonjour, Chef! Welcome back to La Ratatouille Terminal of Taste™!")
    {
        return Err(pixar_fail_joke("starting the CLI welcome message").into());
    }
    Ok(())
}

#[test]
fn test_view_menu_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: View Menu — Checking what’s cooking! 🍽️");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("1\n")
        .assert()
        .stdout(contains("📋 Tonight’s Menu:"))
        .stdout(contains("Ratatouille Supreme"))
        .stdout(contains("Lightning Linguine"))
        .stdout(contains("Fromage Fantastique"));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("displaying tonight’s menu").into());
    }
    Ok(())
}

#[test]
fn test_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Invalid Command — Bananas don’t belong in a terminal! 🙈");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("banana\n")
        .assert()
        .stdout(contains("🐒 Uh... Chef? “banana” is not a valid command."));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("handling invalid fruit commands").into());
    }
    Ok(())
}

#[test]
fn test_close_restaurant() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Close Restaurant — Sweeping the kitchen floors... 👋");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("6\n")
        .assert()
        .stdout(contains("👋 Au revoir! May your soufflés rise"));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("closing the restaurant").into());
    }
    Ok(())
}

#[test]
fn test_add_dish_to_menu() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Add Dish — Experimenting with culinary chaos 🍳");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("2\nToast à la Burnt\ntoast, smoke, regret\n")
        .assert()
        .stdout(contains("✅ “Toast à la Burnt” added to the menu!"));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("adding a new experimental dish").into());
    }
    Ok(())
}

#[test]
fn test_take_new_order() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Take Order — Someone dares to eat Toast à la Burnt! 🧾");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result = cmd
        .write_stdin("3\n5\n4\n")
        .assert()
        .stdout(contains("🧾 Table 5 has ordered “Toast à la Burnt”."));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("placing an order for the brave").into());
    }
    Ok(())
}

#[test]
fn test_advance_order_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Advance Order Status — Cooking up the drama 🔥");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let result1 = cmd
        .write_stdin("5\n5\n")
        .assert()
        .stdout(contains("🍳 Status: Pending → Cooking"));

    if let Err(_) = result1.try_success() {
        return Err(pixar_fail_joke("moving order from pending to cooking").into());
    }

    let mut cmd2 = Command::cargo_bin("ratatui")?;
    let result2 = cmd2
        .write_stdin("5\n5\n")
        .assert()
        .stdout(contains("🍽️ Status: Cooking → Served"));

    if let Err(_) = result2.try_success() {
        return Err(pixar_fail_joke("serving the final dish").into());
    }

    let mut cmd3 = Command::cargo_bin("ratatui")?;
    let result3 = cmd3
        .write_stdin("5\n")
        .assert()
        .stdout(contains("❌ That order is already served, Chef."));

    if let Err(_) = result3.try_success() {
        return Err(pixar_fail_joke("trying to re-cook a memory").into());
    }

    Ok(())
}

#[test]
fn test_overload_easter_egg() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Running Test: Order Overload — Kitchen panic mode! ⚠️");
    let mut cmd = Command::cargo_bin("ratatui")?;

    let mut input = String::from("");
    for i in 1..=6 {
        input += &format!("3\n{}\n4\n", i);
    }
    input += "4\n";

    let result = cmd
        .write_stdin(input)
        .assert()
        .stdout(contains("⚠️ Chef, we’re at MAXIMUM OVERLOAD!"));

    if let Err(_) = result.try_success() {
        return Err(pixar_fail_joke("triggering kitchen panic mode").into());
    }
    Ok(())
}
