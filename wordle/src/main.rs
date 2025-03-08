use std::io;

fn main() {
    let secret_word = "rusty";
    let word_length = secret_word.len();
    let max_attempts = 6;
    
    println!("Welcome to Wordle!");
    println!("Try to guess the {}-letter word.", word_length);
    
    for _ in 0..max_attempts {
        let mut guess = String::new();
        println!("Enter your guess:");
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = guess.trim().to_lowercase();
        
        if guess.len() != word_length {
            println!("Your guess must be {} letters long!", word_length);
            continue;
        }
        
        if guess == secret_word {
            println!("Congratulations! You've guessed the word: {}", secret_word);
            return;
        }
        
        for (i, c) in guess.chars().enumerate() {
            if let Some(secret_char) = secret_word.chars().nth(i) {

                if c == secret_char {
                    print!("[{}]", c);
                }
                else if secret_word.contains(c){
                    print!("({})", c);
                }
                else{
                    print!(" {} ", c);
                }
            }
        }
        println!();
    }
    
    println!("Game over! The word was: {}", secret_word);
}
