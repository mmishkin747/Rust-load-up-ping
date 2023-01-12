use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, process::Child, io, thread::sleep, time::Duration};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("rlup")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("rlup")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


fn run_server(port: u16) -> io::Result<Child> {
    std::process::Command::new("python3").args(["tests/telnet_server.py", port.to_string().as_str()]).spawn()
}

#[test]
fn test_1() -> TestResult {
    let port: u16 = 2061;
    let mut command = run_server(port).unwrap();
    sleep(Duration::from_secs(1));

    let res = run(&["127.0.0.1", "192.168.1.1", "--port", port.to_string().as_str(),], "tests/expected/test_1.txt");

    command.kill().unwrap();
    res
}
