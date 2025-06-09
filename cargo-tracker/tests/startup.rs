use assert_cmd::Command;

fn cargo_fail_joke(context: &str) -> String {
    format!(
        "\n🚚 Oops! Cargo Tracker stumbled while: {}\n💬 Tip: Debug faster than a runaway truck!",
        context
    )
}

#[test]
fn test_startup_and_help_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    let assert = cmd.write_stdin("help\nexit\n").assert();
    let output = String::from_utf8_lossy(&assert.get_output().stdout);

    if !output.contains("Welcome to Cargo Tracker 1.0!")
        || !output.contains("add-shipment")
        || !output.contains("exit")
    {
        return Err(cargo_fail_joke("showing welcome message and help menu").into());
    }

    Ok(())
}
