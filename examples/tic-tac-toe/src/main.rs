use crate::{
    board::{Board, Value},
    minimax::find_best_move,
    moves::Move,
};
use std::io::{self, Write};

mod board;
mod minimax;
mod moves;

fn main() {
    // Start with a clean screen
    clear_screen();
    let mut board = Board::new();
    let player;
    let bot;
    let mut player_turn = true;

    // Loop to ask user which side they want to play as (X or O)
    loop {
        player = match input("Select tile (X or O): ").as_str() {
            "X" => {
                bot = Value::O;
                Value::X
            }
            "O" => {
                bot = Value::X;
                player_turn = false; // Bot moves first if player chooses O
                Value::O
            }
            _ => continue,
        };

        break;
    }

    // Main gameplay loop
    loop {
        clear_screen();
        println!("{board}");

        // Check if game is over
        match board.evaluate_board() {
            board::State::OnGoing => (),
            board::State::Draw => {
                println!("Draw!");
                return;
            }
            board::State::XWins => {
                println!(
                    "{}",
                    match player {
                        Value::X => "You Win!",
                        Value::O => "You Lose!",
                        _ => unreachable!(),
                    }
                );
                return;
            }
            board::State::OWins => {
                println!(
                    "{}",
                    match player {
                        Value::X => "You Lose!",
                        Value::O => "You Win!",
                        _ => unreachable!(),
                    }
                );
                return;
            }
        }

        // Handle either the player's turn or the bot's turn
        if player_turn {
            // Print the available options
            println!("(1) Top Left");
            println!("(2) Top");
            println!("(3) Top Right");
            println!("(4) Left");
            println!("(5) Center");
            println!("(6) Right");
            println!("(7) Bottom Left");
            println!("(8) Bottom");
            println!("(9) Bottom Right");
            println!();

            // Prompt user input for move index (1-9)
            let move_i: usize = match input("What is your move? ").parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Convert to 0-indexed enum variant
            let move_v = Move::from(move_i - 1);

            // Attempt to place the piece on the board
            match player {
                Value::O => {
                    if board.place_o(move_v).is_none() {
                        continue; // Cell is already occupied or invalid
                    }
                }
                Value::X => {
                    if board.place_x(move_v).is_none() {
                        continue; // Cell is already occupied or invalid
                    }
                }
                _ => unreachable!(),
            };

            player_turn = false;
        } else {
            // Calculate best move using Minimax algorithm
            let best_move = find_best_move(&board, bot).unwrap();

            // Place bot's piece
            match bot {
                Value::O => board.place_o(best_move),
                Value::X => board.place_x(best_move),
                _ => unreachable!(),
            };

            player_turn = true;
        }
    }
}

// Reads standard input from the user after writing a prompt.
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

// Clears the terminal screen using ANSI escape sequences.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
