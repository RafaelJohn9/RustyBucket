use assert_cmd::Command;
use predicates::str::contains;

fn cargo_fail_joke(context: &str) -> String {
    format!(
        "\n📋 View/List glitch: {}\n💬 Even lists deserve tests.",
        context
    )
}

#[test]
fn test_update_status_success_and_not_found_and_invalid() -> Result<(), Box<dyn std::error::Error>>
{
    // Setup shipment
    let setup_input = "add-shipment\nLMN111\nKisumu\nq\n";

    // Update valid
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    cmd.write_stdin(format!(
        "{setup_input}update-status\nLMN111\nInTransit\nexit\n"
    ))
    .assert()
    .stdout(contains("Shipment 'LMN111' status updated to InTransit."));

    // Shipment not found
    let mut cmd2 = Command::cargo_bin("cargo-tracker")?;
    cmd2.write_stdin(format!("{setup_input}update-status\nZZZ999\nexit\n"))
        .assert()
        .stdout(contains("Shipment with Tracking ID 'ZZZ999' not found."));

    // Invalid status
    let mut cmd3 = Command::cargo_bin("cargo-tracker")?;
    cmd3.write_stdin(format!(
        "{setup_input}update-status\nLMN111\nShipped\nexit\n"
    ))
    .assert()
    .stdout(contains(
        "Error: Invalid status. Valid options are: Pending, InTransit, Delivered, Lost.",
    ));

    Ok(())
}
