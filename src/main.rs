use std::process::Command;
use rand::Rng;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program",
    ];
    let index = rand::thread_rng().gen_range(0..messages.len());
    let message = messages[index];
    let result = Command::new("git").args(["pull"]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["add", "."]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["commit", "-m", message]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["push"]).output();
    println!("{:?}", result);
}