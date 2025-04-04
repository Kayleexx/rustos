use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn play_game(args: Vec<&str>) {
    if args.get(1) == Some(&"hangman") {
        hangman();
    } else {
        println!("Available games: hangman");
    }
}

fn hangman() {
    let words = ["crab", "rust", "compiler", "ownership"];
    let word = words[rand::thread_rng().gen_range(0..words.len())].to_lowercase();
    let mut guessed = vec!['_'; word.len()];
    let mut tries = 6;
    let mut used_letters = HashSet::new();

    while tries > 0 {
        println!("\nWord: {}", guessed.iter().collect::<String>());
        println!("Tries left: {}", tries);
        print!("Guess a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().to_lowercase();

        // Validate input: single letter only
        if guess.len() != 1 || !guess.chars().all(|c| c.is_alphabetic()) {
            println!("⚠️ Please enter a single letter.");
            continue;
        }

        let guess_char = guess.chars().next().unwrap();

        // Check if letter was already guessed
        if used_letters.contains(&guess_char) {
            println!("⚠️ You already guessed '{}'. Try again!", guess_char);
            continue;
        }

        used_letters.insert(guess_char);

        if word.contains(guess_char) {
            for (i, c) in word.chars().enumerate() {
                if c == guess_char {
                    guessed[i] = c;
                }
            }
        } else {
            println!("❌ Wrong guess!");
            tries -= 1;
        }

        if !guessed.contains(&'_') {
            println!("🎉 You win! The word was '{}'.", word);
            return;
        }
    }

    println!("💀 You lost! The word was '{}'.", word);
}
