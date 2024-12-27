use std::io::{stdin, stdout, Write};

use domain::{board::Board, player::Player, position::Position, GameState};

mod domain;

fn prompt_for_input(prompt: String) -> String {
    print!("{prompt}");
    stdout().flush().expect("Flush failed");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn get_next_move(player: &Player) -> Position {
    match prompt_for_input(String::from(format!("\n{player} to play: "))).parse::<Position>() {
        Ok(m) => m,
        Err(err) => {
            println!("{err}");
            get_next_move(player)
        }
    }
}

fn main() {
    let mut board = Board::new();
    println!("Terminal tic-tac-toe!");
    println!("{}", board);

    let mut move_number = 1;
    loop {
        let current_player = Player::from_move(move_number);
        let next_move = get_next_move(&current_player);
        match board.play_move(&current_player, next_move) {
            Ok(GameState::Winner(winner)) => {
                println!("{board}");
                println!("Congratulations! Player {winner} won!");
                break;
            }
            Ok(GameState::NoWinner) => {
                println!("{board}");
                println!("It's a draw!");
                break;
            }
            Ok(GameState::InProgress) => {
                println!("{board}");
            }
            Err(err) => println!("{}", err.message),
        }
        move_number = move_number + 1;
    }
}
