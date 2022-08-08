const WORD: &str = "girl";

fn main() {
    std::io::stdin().lines().for_each(|guess| {
        let correct_letters: String = guess
            .unwrap()
            .chars()
            .filter(|&c| WORD.contains(c))
            .collect();

        if WORD == correct_letters {
            println!("You Win!");
            std::process::exit(0);
        } else {
            println!("{}", correct_letters);
        }
    });
}
