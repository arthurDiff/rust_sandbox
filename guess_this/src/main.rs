use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    'game: loop {
        println!("Let's start this guessing game! Guess the number!");

        let sec_num = rand::thread_rng().gen_range(0..=100);
        println!("Please input your guess between 0 to 100.");
        'innerGame: loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to readline");
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide a valid input!");
                    continue;
                }
            };

            let won = match guess.cmp(&sec_num) {
                Ordering::Less => {
                    println!(
                        "Too Small :< {}",
                        if guess < 0 {
                            "Why you picking below zero!"
                        } else {
                            "Try Again!"
                        }
                    );
                    false
                }
                Ordering::Equal => {
                    println!("You Win!!!");
                    true
                }
                Ordering::Greater => {
                    println!(
                        "Too Large :< {}",
                        if guess > 100 {
                            "Why you picking over 100!"
                        } else {
                            "Try Again!"
                        }
                    );
                    false
                }
            };

            if won {
                println!("Play again enter 'a | A'. or Quit by enterig 'q | Q'!");
                let mut replay_answer = String::new();
                io::stdin()
                    .read_line(&mut replay_answer)
                    .expect("Sorry unexpected error occured");

                match replay_answer.trim().to_lowercase().as_str() {
                    "a" => {
                        println!("Starting new game");
                        break 'innerGame;
                    }
                    _ => {
                        println!("Thank you for playing!");
                        break 'game;
                    }
                }
            }
        }
    }
}
