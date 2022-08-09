const WORD: &str = "girl";
const MAX_GUESSES: u8 = 6;

fn main() {
    let mut tries = 0;
    std::io::stdin().lines().for_each(|guess| {
        let guess = guess.unwrap();

        // Check if guess length is not the same as word length
        if guess.len() != WORD.len() {
            println!("Word length is {} letters!", WORD.len());
            return;
        }

        // Check if tries is larger than max guesses
        if tries >= MAX_GUESSES {
            println!("You Lost! The word was {}", WORD);
            std::process::exit(0);
        }

        // Filter the out the correct letters from the guess
        let correct_letters: String = guess
            .chars()
            .filter(|&letter| WORD.contains(letter))
            .collect();

        if correct_letters == WORD {
            println!("You Win!");
            std::process::exit(0);
        } else {
            tries += 1;
            println!("Correct letters: {}", correct_letters);
            println!("You have {} guesses remaining", MAX_GUESSES - tries);
        }
    });
}
