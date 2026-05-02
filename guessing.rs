use std::io;
use rand::Rng;
pub fn guessing_game() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let rand_num = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin().read_line(& mut guess).expect("fail line");
    println!("You guessed: {}", guess);
    if guess.trim() == "quit" {
        println!("bye bye");
        return;
    }
    if guess.trim() == rand_num.to_string() {
        println!("You guessed right! Play again? Y/N");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if(answer.trim() == "Y".to_lowercase() || answer.trim() == "Y") {
            guessing_game();
        }
        else if(answer.trim() == "N".to_lowercase() || answer.trim() == "N") {
            println!("thanks for playing!");
            return;
        }
        else{
            println!("Please input Y or N");
            guess = String::new();
        }
    }
    else {
        println!("Wrong! The number was {}", rand_num);
        println!("Play again? Y/N");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if(answer.trim() == "Y".to_lowercase() || answer.trim() == "Y") {
            guessing_game();
        }
        else if(answer.trim() == "N".to_lowercase() || answer.trim() == "N") {
            println!("thanks for playing!");
            return;
        }
        else{
            println!("Please input Y or N");
            guess = String::new();
        }

    }


}