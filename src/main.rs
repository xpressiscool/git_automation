use std::process::Command;
use rand::Rng;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program",
        "i love git",
        "i love programming"
    ];
    // gets random number integer
    let index = rand::thread_rng().gen_range(0..messages.len());
    // sets the commit message to a random element in the messages vector
    let message = messages[index];

    // pulls the code from repo
    let result = Command::new("git").args(["pull"]).output();
    println!("{:?}", result);

    // stages the changes
    let result = Command::new("git").args(["add", "."]).output();
    println!("{:?}", result);

    // commits the changes to the repository
    let result = Command::new("git").args(["commit", "-m", message]).output();
    println!("{:?}", result);

    // pushes all of the commits to the repository
    let result = Command::new("git").args(["push"]).output();
    println!("{:?}", result);
}