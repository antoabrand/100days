use std::cmp::Ordering;
use std::io;

    pub fn play_game(secret_num: u32) {
        loop {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Fail to read line");
    
            println!("Your guess: {guess}");
            let guess: u32 = guess.trim().parse().expect("Expected a number");
    
            println!("Secret Number: {}", secret_num);
    
            match guess.cmp(&secret_num) {
                Ordering::Equal => {
                    println!("You found it!");
                    break;
                }
                Ordering::Less => println!("To little..."),
                Ordering::Greater => println!("Too big..."),
            }
        }
    }


