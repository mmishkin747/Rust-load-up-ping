use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, process::Child, io};

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

    Command::cargo_bin("rfd")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


fn run_server(port: u16) -> io::Result<Child> {
    std::process::Command::new("python3").args(["./telnet_server", port.to_string().as_str()]).spawn()
}

#[test]
fn test_1() -> TestResult {
    let port: u16 = 2065;
    let command = run_server(port);

    let res = run(&["tests/input/hello.docx"], "tests/expected/hello.txt");

    if let Ok(mut child) = command {
        child.kill().expect("command wasn't running");
    } else {
        println!("yes command didn't start");
    };
    res
}
