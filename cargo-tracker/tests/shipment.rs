use assert_cmd::Command;
use predicates::str::contains;

fn cargo_fail_joke(context: &str) -> String {
    format!("\n🚚 Cargo hiccuped while: {}\n💬 Try again from the loading dock!", context)
}

#[test]
fn test_add_shipment_and_duplicate() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    let assert1 = cmd
        .write_stdin("add-shipment\nABC123\nNairobi\nElectronics\nq\nexit\n")
        .assert()
        .stdout(contains("Shipment Created"));

    if let Err(_) = assert1.try_success() {
        return Err(cargo_fail_joke("creating new shipment ABC123").into());
    }

    let mut cmd2 = Command::cargo_bin("cargo-tracker")?;
    let assert2 = cmd2
        .write_stdin("add-shipment\nABC123\nMombasa\nq\nexit\n")
        .assert()
        .stdout(contains("Error: Shipment with tracking ID 'ABC123' already exists."));

    if let Err(_) = assert2.try_success() {
        return Err(cargo_fail_joke("detecting duplicate shipment ABC123").into());
    }

    Ok(())
}
