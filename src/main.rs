const WORD: &str = "girl";

fn main() {
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
            println!("Correct letters: {}", correct_letters);
        }
    });
}
