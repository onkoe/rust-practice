use core::panic;

use colored::*;
use dialoguer::Input;
use rand::Rng;

#[derive(Debug)]
/// Am enum representing Rock Paper Scissors moves.
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Represents all rules for RPS moves.
    fn play_round(&self, cpu_move: &Move) -> Winner {
        match self {
            Move::Rock => match cpu_move {
                Move::Rock => Winner::NoOne,
                Move::Paper => Winner::Computer,
                Move::Scissors => Winner::Player,
            },
            Move::Paper => match cpu_move {
                Move::Rock => Winner::Player,
                Move::Paper => Winner::NoOne,
                Move::Scissors => Winner::Computer,
            },
            Move::Scissors => match cpu_move {
                Move::Rock => Winner::Computer,
                Move::Paper => Winner::Player,
                Move::Scissors => Winner::NoOne,
            },
        }
    }
}

#[derive(Debug)]
/// Represents who won the game.
enum Winner {
    Player,   // booooooo!
    Computer, // you got this baby. i believe in u 🥰️🥵️
    NoOne,    // :(
}

// Represents computer as a player.
struct Computer {
    cpu_move: Move,
    lose_count: u32,
}

impl Computer {
    /// Randomly generates a new move for the computer.
    fn get_move(&mut self) {
        let mut rng = rand::thread_rng();

        self.cpu_move = match rng.gen_range(0..=2) {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        };
    }

    /// C̸͇̊h̸̞͑e̷̪͠c̸̩͂k̴͇̓s̵͙̑ ̵͎̏t̶͍̀ò̶̠ ̶̤̊s̷͈͒e̴̦͌e̶̩͘ ̷̝͗i̷̩͝f̶̆ͅ ̵͉̑t̷̟̚h̶̟̔ẽ̸̝ ̶͎̇c̴̹̆ȯ̴̙m̷̧̀p̴̦̈́u̵͍̽t̵͆͜e̶̯̚ȑ̷̡ ̴̢͘ẁ̴̯i̷̖͋l̶̳͂ľ̵̟ ̸̛̮c̷̲̾a̷̯͝u̵̦̓s̶̞̒ë̸̙ ̵̛̰t̷̗͊ḩ̴̾e̷̘͠ ̷̫̚ṕ̶͖r̷̰̾ȍ̶͍g̴̤̓r̴̥͝a̴̬͊m̴̠͒ ̸̞͘ẗ̶̼o̴̼̅ ̷̗͋p̸̼̊a̸̝̅n̸͍͛i̷̢̾c̶̟̀.̵͉̃
    /// There's nothing here! :)
    fn upset(&self) -> bool {
        let mut rng = rand::thread_rng();

        let number: u32 = rng.gen_range(self.lose_count..=20);

        match number {
            20 => true,
            _ => false,
        }
    }

    /// Creates a new Computer instance.
    fn computer() -> Computer {
        Computer {
            cpu_move: Move::Rock,
            lose_count: 0,
        }
    }
}

fn main() {
    let mut computer = Computer::computer();
    let mut user_move: Move;
    let mut game_result: Winner;

    // Welcome message
    println!("\n{}", "Welcome to rock_paper_scissors!".bright_green());
    println!("{}", "Here, you try to beat the vicious computer!".green());
    println!(
        "{}{}{}",
        "To begin, type".bright_cyan(),
        " rock, paper, or scissors".red(),
        ", then hope you're right!".bright_cyan()
    );
    println!();

    // Game loop
    loop {
        computer.get_move();

        user_move = get_user_move();
        println!();

        // Tell user who picked what
        println!(
            "{}",
            (format!(
                "The computer gave {}, and you gave {}!",
                format!("{:#?}", &computer.cpu_move).red(),
                format!("{:#?}", &user_move).green()
            ))
        );

        game_result = user_move.play_round(&computer.cpu_move);

        // Print game result with messages from computer.
        match game_result {
            Winner::Player => {
                println!("{}", "Good job! You won! 😭️".red());
                computer.lose_count += 1;
                if computer.upset() == true {
                    panic!("Go away...")
                }
            }
            Winner::Computer => println!("{}", "Hey, I won. Good try, though!".green()),
            Winner::NoOne => println!(
                "{}\n{}",
                "Neither of us won? Great minds think alike!".green(),
                "Or I'm having an off-day...".black()
            ),
        }
    }
}

/// Prompts user to input a move.
fn get_user_move() -> Move {
    let mut user_input: Result<String, std::io::Error>;
    loop {
        user_input = Input::new().interact();
        match user_input {
            Ok(mov) => match try_to_move(mov) {
                Some(mov) => return mov,
                None => {
                    continue;
                }
            },
            Err(_) => {
                continue;
            }
        }
    }

    /// Attempts to get user move from given String. 
    fn try_to_move(user_input: String) -> Option<Move> {
        match user_input.trim().to_lowercase().as_str() {
            "r" | "rock" => Some(Move::Rock),
            "p" | "paper" => Some(Move::Paper),
            "s" | "scissors" => Some(Move::Scissors),
            "sissors" | "sisors" | "scisors" | "scizzors" => {
                println!("hey, it's \"scissors\" buddy!");
                Some(Move::Scissors)
            }
            _ => None,
        }
    }
}
