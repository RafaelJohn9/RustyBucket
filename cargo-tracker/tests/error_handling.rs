use assert_cmd::Command;

fn cargo_fail_joke(context: &str) -> String {
    format!(
        "\n❗ Error handling fail: {}\n💬 Let's secure those edge cases!",
        context
    )
}

#[test]
fn test_invalid_command_and_filter_no_matches() -> Result<(), Box<dyn std::error::Error>> {
    // Test invalid command
    let mut cmd = Command::cargo_bin("cargo-tracker")?;
    let assert = cmd.write_stdin("banana\nexit\n").assert();

    let output = String::from_utf8(assert.get_output().stdout.clone())?;
    if !output.contains("is not a valid command") {
        return Err(cargo_fail_joke(&format!(
            "invalid command handling\n--- stdout was:\n{}",
            output
        ))
        .into());
    }

    // Test filter with no matches
    let mut cmd2 = Command::cargo_bin("cargo-tracker")?;
    let input = "list-shipments\nZZZ999\nexit\n";
    let assert2 = cmd2.write_stdin(input).assert();

    let output2 = String::from_utf8(assert2.get_output().stdout.clone())?;
    if !output2.contains("No shipments found for status 'ZZZ999'") {
        return Err(cargo_fail_joke(&format!(
            "listing with no matches\n--- stdout was:\n{}",
            output2
        ))
        .into());
    }

    Ok(())
}
