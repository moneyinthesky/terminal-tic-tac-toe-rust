use player::Player;

pub mod board;
pub mod position;
pub mod player;
pub mod square;

#[derive(PartialEq, Debug)]
pub struct GameError { pub message: String }

#[derive(PartialEq, Debug)]
pub enum GameState {
    Winner(Player),
    NoWinner,
    InProgress,
}
