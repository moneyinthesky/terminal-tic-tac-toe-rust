use std::io::{stdin, stdout, Error, Write};

use console::{style, Term};
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
    match prompt_for_input(String::from(format!("\n{} to play: ", player.styled()))).parse::<Position>() {
        Ok(position) => position,
        Err(err) => {
            println!("{err}");
            get_next_move(player)
        }
    }
}

fn main() -> Result<(), Error> {
    let term = Term::stdout();

    let mut board = Board::new();

    println!("{}", style("Terminal tic-tac-toe!").bold().cyan());
    println!("{}", style("Select square in format \"ROW,COL\"").dim().italic());
    println!("{}", board);

    let mut lines_to_clear = 9;
    let mut move_number = 1;
    loop {
        let current_player = Player::from_move(move_number);
        let next_move = get_next_move(&current_player);
        match board.play_move(&current_player, next_move) {
            Ok(GameState::Winner(winner)) => {
                term.clear_last_lines(lines_to_clear)?;
                println!("{board}");
                println!("Congratulations! Player {} won!", winner.styled());
                break;
            }
            Ok(GameState::NoWinner) => {
                term.clear_last_lines(lines_to_clear)?;
                println!("{board}");
                println!("{}", style("It's a draw!").yellow().bold());
                break;
            }
            Ok(GameState::InProgress) => {
                move_number = move_number + 1;
                term.clear_last_lines(lines_to_clear)?;
                lines_to_clear = 9;
                println!("{board}");
            }
            Err(err) => {
                lines_to_clear = lines_to_clear + 3;
                println!("{}", err.message);
            }
        }
    }
    Result::Ok(())
}
