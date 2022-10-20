mod play;
mod rando;

use play::play_game;
use rando::generate_random_num;
fn main() {
    println!("Guess a number between 1 and 100...");
    play_game(generate_random_num(1, 100));
}
