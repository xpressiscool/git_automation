use std::process::Command;
use rand::prelude::SliceRandom;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program",
    ];
    let message: &str = messages 
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();

    // generates random index for a message to commit
    let result = Command::new("git").args(["pull"]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["add", "."]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["commit", "-m", message]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["push"]).output();
    println!("{:?}", result);
}
