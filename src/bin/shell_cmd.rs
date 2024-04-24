use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Specify the shell command to run
    let command = "curl --unix-socket /var/run/docker.sock -X GET http:/v1.45/containers/json";

    // Create a Command instance
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    // Print the command output
    io::stdout().write_all(&output.stdout).unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    // Print the command output
    println!("{}", output);

    // Print the command error (if any)
    // io::stderr().write_all(&output.stderr).unwrap();
}
