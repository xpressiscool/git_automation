use rand::Rng;
use std::process::Command;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program",
    ];

    // generates random index for a message to commit
    let mut result = rand::thread_rng().gen_range(0..messages.len());
    let message = messages[index];

    let mut add = Command::new("git")
        .args(["add", "."])
        .output();
    println!("{:?}", add);
    let mut result = Command::new("git")
        .args(["commit", "-m", "changes"])
        .output();
    println!("{:?}", result);
    let mut push = Command::new("git")
        .args(["push"])
        .output();
    println!("{:?}", push);
}
