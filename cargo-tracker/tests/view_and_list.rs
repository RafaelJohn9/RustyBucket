use assert_cmd::Command;
use predicates::str::contains;

fn cargo_fail_joke(context: &str) -> String {
    format!(
        "\n📋 View/List glitch: {}\n💬 Even lists deserve tests.",
        context
    )
}

#[test]
fn test_view_shipment_and_list_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    let setup = r"\
add-shipment\nAAA222\Nairobi\np\nfragile\nq\nadd-shipment\nBBB333\nMombasa\nq\n";
    cmd.write_stdin(setup).assert();

    // View AAA222
    let mut cmd2 = Command::cargo_bin("cargo-tracker")?;
    let view_input = "view-shipment\nAAA222\nexit\n";
    let assert_view = cmd2
        .write_stdin(view_input)
        .assert()
        .stdout(contains("Tracking ID: AAA222"))
        .stdout(contains("Destination: Nairobi"))
        .stdout(contains("fragile"));

    if let Err(_) = assert_view.try_success() {
        return Err(cargo_fail_joke("viewing shipment AAA222").into());
    }

    // List all shipments
    let mut cmd3 = Command::cargo_bin("cargo-tracker")?;
    let assert_list = cmd3
        .write_stdin("list-shipments\nexit\n")
        .assert()
        .stdout(contains("Tracking ID: AAA222"))
        .stdout(contains("Tracking ID: BBB333"));

    if let Err(_) = assert_list.try_success() {
        return Err(cargo_fail_joke("listing all shipments").into());
    }

    Ok(())
}
