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
    println!("{:?}", result);
    let result = Command::new("git").args(["commit", "-m more changes"]).output();
    println!("{:?}", result);
    let result = Command::new("git").args(["push"]).output();
    println!("{:?}", result);
}
