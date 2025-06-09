use assert_cmd::Command;
use predicates::str::contains;

fn cargo_fail_joke(context: &str) -> String {
    format!("\n📦 Package chaos: {}\n💬 Pro tip: even packages deserve good tests!", context)
}

#[test]
fn test_add_package_to_existing_shipment() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    let input = "add-shipment\nXYZ999\nMombasa\na\nElectronics\nBooks\nq\nadd-package\nXYZ999\nHeadphones\nexit\n";
    let assert = cmd.write_stdin(input).assert().stdout(contains(
        "Package added to shipment 'XYZ999'.",
    ));

    if let Err(_) = assert.try_success() {
        return Err(cargo_fail_joke("adding package to XYZ999").into());
    }

    Ok(())
}
