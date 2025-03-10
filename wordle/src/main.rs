use std::io;

fn main() {
    let secret_word = "rusty";
    let word_length = secret_word.len();
    let max_attempts = 6;
    
    println!("Witaj w Wordle!");
    println!("Sprobuj zgadnac {} literowe slowo", word_length);
    
    for _ in 0..max_attempts {
        let mut guess = String::new();
        println!("Wprowadz slowo:");
        io::stdin().read_line(&mut guess).expect("Blad odczytu");
        let guess = guess.trim().to_lowercase();
        
        if guess.len() != word_length {
            println!("Twoje slowo musi byc {} literowe!", word_length);
            continue;
        }
        
        if guess == secret_word {
            println!("Gratulacje zgadles: {}", secret_word);
            return;
        }
        
        for (i, c) in guess.chars().enumerate() {
            let secret_char = secret_word.chars().nth(i).unwrap();
            if c == secret_char {
                print!("[{}]", c);
            } 
            else if secret_word.contains(c) {
                print!("({})", c);
            } 
            else {
                print!(" {} ", c);
            }
        }
        println!();
    }
    
    println!("Koniec gry! Haslo to: {}", secret_word);
}
