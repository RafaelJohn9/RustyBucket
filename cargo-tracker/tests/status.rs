use assert_cmd::Command;
use predicates::str::contains;

fn cargo_fail_joke(context: &str) -> String {
    format!(
        "\n🚛 Status slip-up: {}\n💬 Check your state machine, friend.",
        context
    )
}

#[test]
fn test_update_status_success_and_not_found_and_invalid() -> Result<(), Box<dyn std::error::Error>>
{
    // Setup shipment
    let setup_input = "add-shipment\nLMN111\nKisumu\nq";

    // Update valid
    let mut cmd2 = Command::cargo_bin("cargo-tracker")?;
    let assert_valid = cmd2
        .write_stdin(format!(
            "{setup_input}\nupdate-status\nLMN111\nInTransit\nexit\n"
        ))
        .assert()
        .stdout(contains("Shipment 'LMN111' status updated to InTransit."));

    if let Err(_) = assert_valid.try_success() {
        return Err(cargo_fail_joke("updating valid shipment status").into());
    }

    // Shipment not found
    let mut cmd3 = Command::cargo_bin("cargo-tracker")?;
    let assert_missing = cmd3
        .write_stdin(format!("{setup_input}\nupdate-status\nZZZ999\nexit\n"))
        .assert()
        .stdout(contains("Shipment with Tracking ID 'ZZZ999' not found."));

    if let Err(_) = assert_missing.try_success() {
        return Err(cargo_fail_joke("handling missing shipment LMN111").into());
    }

    // Invalid status
    let mut cmd4 = Command::cargo_bin("cargo-tracker")?;
    let assert_invalid = cmd4
        .write_stdin(format!(
            "{setup_input}\nupdate-status\nLMN111\nShipped\nexit\n"
        ))
        .assert()
        .stdout(contains(
            "Error: Invalid status. Valid options are: Pending, InTransit, Delivered, Lost.",
        ));

    if let Err(_) = assert_invalid.try_success() {
        return Err(cargo_fail_joke("detecting invalid status value").into());
    }

    Ok(())
}
