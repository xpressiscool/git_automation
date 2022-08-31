use std::process::Command;

fn main() {
    let messages = vec![
        "it might work",
        "ooh new feature!",
        "might break the program"
    ];

    // generates random index for a message to commit
    let mut index = rand::thread_rng();
    let mut add = Command::new("git add .");
    let mut commit = Command::new("git commit -m {}", rng.gen_range(0..messages.len()));
}
