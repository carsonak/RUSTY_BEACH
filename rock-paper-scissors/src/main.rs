use std::io;

fn main() {
    println!("Rock! Paper! Scissors!");

    let mut player = String::new();
    io::stdin()
        .read_line(&mut player)
        .expect("read_line failure");

    println!("Your play: {player}")
}
