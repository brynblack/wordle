use rand::Rng;
use std::{fs, io, process};

const FILE_NAME: &str = "words";
const MAX_GUESSES: u8 = 6;

fn main() {
    let contents = fs::read_to_string(FILE_NAME).unwrap();
    let words: Vec<&str> = contents.lines().collect();
    let index: usize = rand::thread_rng().gen_range(0..words.len());
    let word = words[index];

    let mut tries = 0;
    io::stdin().lines().for_each(|guess| {
        let guess = guess.unwrap();

        if guess.len() != word.len() {
            println!("Word length is {} letters!", word.len());
            return;
        }

        let letters_in_place: String = guess
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if word.chars().nth(i).unwrap() == c {
                    c
                } else {
                    'X'
                }
            })
            .collect();

        let correct_letters: String = guess
            .chars()
            .filter(|&letter| word.contains(letter))
            .collect();

        if correct_letters == word {
            println!("You Win!");
            process::exit(0);
        } else {
            tries += 1;

            if tries >= MAX_GUESSES {
                println!("You Lost! The word was {}", word);
                process::exit(0);
            }

            println!("Correct letters: {}", correct_letters);
            println!("Letters in place: {}", letters_in_place);
            println!("You have {} guesses remaining", MAX_GUESSES - tries);
        }
    });
}
