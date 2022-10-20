use std::cmp::Ordering;
use std::io;

pub fn play_game(secret_num: u32) {
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number...");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("****************************************");
                println!("****************************************");
                println!("You found it!");
                println!("The secret number was: {}", secret_num);
                println!("****************************************");
                println!("****************************************");
                break;
            }
            Ordering::Less => println!("To small, try larger..."),
            Ordering::Greater => println!("Too big, try smaller..."),
        }
    }
}
