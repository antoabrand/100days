use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Guessing game!");
    println!("Please guess a number");
    let secret_number = generate_random_num(); 
    let mut guess = String::new();
    let mut answered_correctly : bool = false; 

    io::stdin()
    .read_line(&mut guess)
    .expect("Fail to read line");

    println!("Your guess: {guess}");
    let mut num : i32 = guess.trim().parse().unwrap();

    while answered_correctly == false {
        answered_correctly =  check_guess(secret_number, num); 
        io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");
    }
   
}

fn generate_random_num() -> i32 {
    return rand::thread_rng().gen_range(1..=100);
}

fn check_guess(secret_num : i32, guess : i32) -> bool {

    let mut guessed_num = guess;

    println!("Secret number is : {}", secret_num);
    

    if secret_num == guessed_num {
        println!("Congrats!  You guessed correctly : {}!", guess);
        return true; 
    }

    if  secret_num < guessed_num {
        println!("Try smaller ...");
    } 
    
    if secret_num > guessed_num {
        println!("Try larger ... ");
    }

    return false; 

}
