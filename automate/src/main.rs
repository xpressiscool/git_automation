use rand::Rng;
use std::process::Command;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program",
    ];

    // generates random index for a message to commit
    let result = Command::new("git").args(["pull"]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["add", "."]).output();
    println!("{:?}", add);
    let result = Command::new("git").args(["commit", "-m changes"]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["push"]).output();
    println!("{:?}", push);
}
