use rand::prelude::*;
use std::env::*;
use std::fs::File;
use std::hash;
use std::io::*;
use std::process;
use std::ptr::hash;

fn main() {
    println!("Welcome to Hangman! Type 'quit' to leave at any time!");
    hangman();
}

fn hangman() -> bool {
    let allowed_wrong_guesses: u32 = 5; // adjust this value to make the game more or less difficult
    let mut wrong_guesses: u32 = 0;

    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let word = generate_word();
    let mut hashed_word = String::from_utf8(vec![b'#'; word.len()]).unwrap();

    // debugging printlns
    println!("hashed word: {}", hashed_word);
    println!("psst. the word is: {}", word);

    loop {
        if wrong_guesses >= allowed_wrong_guesses {
            println!("You're out of guesses!");
            break;
        }

        // input
        let guess = get_user_input();

        // check for quit in user input
        if guess == "quit".to_string() {
            break;
        }

        if guess == word {
            println!("You won! Good job!");
            break;
        }

        let mut guessedLetter = guess.chars().nth(0).unwrap();

        // check is guessed letter ISN'T in word
        if !word.contains(guessedLetter) {
            wrong_guesses += 1;
        }

        // let new hashed word = hashed word but as a vector :)
        let mut new_hashed_word: Vec<char> = hashed_word.chars().collect();

        // check if the word has the guessed letter
        // if so, replace the hash with guessed letter
        // else keep the hash

        // also make sure to make the hashed word now equal to the new_hashed_word

        // check to ENSURE that new hashed word is cleaned with each loop! 

        

        if word.contains(guessedLetter) {
            for i in word.to_string().chars() {
                if i == guessedLetter {
                    new_hashed_word. = guessedLetter;
                    println!("yo they guessed it, here's new hashed word: {:?}", new_hashed_word);
                } else {
                    new_hashed_word[i] = '#';
                    println!("{} is guess letter. lmao they didn't guess it, new hashed word is: {:?}", &guessedLetter, new_hashed_word);
                }
            }
        }
        /* old version of above
        let mut new_hashed_word = hashed_word;
        if word.contains(guessedLetter) {
            for char in word.chars() {
                if char == guessedLetter {
                    new_hashed_word.push(char);
                } else {
                    new_hashed_word.push('#');
                    wrong_guesses += 1;
                }
            }
        } */

        hashed_word = new_hashed_word.into_iter().collect();

        println!("{}", hashed_word);
    }

    true
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read user input!");
    user_input.trim_end().to_string()
}

fn generate_word() -> String {
    let mut rng = rand::thread_rng();

    let mut f = File::open("dictionary.txt").expect("Dictionary file was not found!");
    let mut dictionary = String::new();
    f.read_to_string(&mut dictionary)
        .expect("Dictionary file is not a newline separated file. It may be damaged!");

    // puts all words into a vector, then grabs a random one
    let words: Vec<&str> = dictionary.split("\n").collect();
    let index = rng.gen_range(0..words.len());

    let word = words[index];

    return word.to_string();
}
