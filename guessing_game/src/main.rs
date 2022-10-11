mod rando;
mod play;

use rando::generate_random_num;
use play::play_game;
fn main() {
    println!("Hello, Guessing game!");
    println!("Please guess a number");
    play_game(generate_random_num(1,100));
}