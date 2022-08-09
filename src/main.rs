const WORD: &str = "girl";
const MAX_GUESSES: u8 = 6;

fn main() {
    let mut tries = 0;
    std::io::stdin().lines().for_each(|guess| {
        let guess = guess.unwrap();

        if guess.len() != WORD.len() {
            println!("Word length is {} letters!", WORD.len());
            return;
        }

        let correct_letters: String = guess
            .chars()
            .filter(|&letter| WORD.contains(letter))
            .collect();

        if correct_letters == WORD {
            println!("You Win!");
            std::process::exit(0);
        } else {
            tries += 1;

            if tries >= MAX_GUESSES {
                println!("You Lost! The word was {}", WORD);
                std::process::exit(0);
            }

            println!("Correct letters: {}", correct_letters);
            println!("You have {} guesses remaining", MAX_GUESSES - tries);
        }
    });
}
