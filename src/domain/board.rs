use std::fmt::Display;

use super::{player::Player, position::Position, square::Square, GameError, GameState};

#[derive(PartialEq, Debug, Default)]
pub struct Board {
    pub squares: [[Square; 3]; 3],
}

impl Board {
    pub fn play_move(
        &mut self,
        player: &Player,
        position: Position,
    ) -> Result<GameState, GameError> {
        match self.get_position(&position) {
            Square::EMPTY => {
                self.set_position(&position, player);

                if self.check_if_player_won(player) {
                    Result::Ok(GameState::Winner(player.clone()))
                } else if self.board_full() {
                    Result::Ok(GameState::NoWinner)
                } else {
                    Result::Ok(GameState::InProgress)
                }                
            },
            current => {
                Result::Err(GameError {
                    message: format!("Player {} has already played in this square. Please select a different square.", current),
                })
            }
        }
    }

    pub fn check_if_player_won(&self, player: &Player) -> bool {
        (1..=3).any(|row_number| self.check_row(player, row_number))
            || (1..=3).any(|col_number| self.check_col(player, col_number))
            || self.check_diagonals(player)
    }

    fn get_position(&self, position: &Position) -> &Square {
        &self.squares[(position.0 as usize) - 1][(position.1 as usize) - 1]
    }

    fn set_position(&mut self, position: &Position, player: &Player) {
        self.squares[(position.0 as usize) - 1][(position.1 as usize) - 1] =
            Square::from_player(player);
    }

    fn board_full(&self) -> bool {
        self.squares
            .iter()
            .all(|row| row.iter().all(|square| square.is_filled()))
    }

    fn matches(&self, position: &Position, player: &Player) -> bool {
        match &self.get_position(position) {
            Square::EMPTY => false,
            p => p.filled_by(player),
        }
    }

    fn check_col(&self, player: &Player, col_number: u8) -> bool {
        (1..=3).all(|row_number| self.matches(&Position(row_number, col_number), player))
    }

    fn check_row(&self, player: &Player, row_number: u8) -> bool {
        (1..=3).all(|col_number| self.matches(&Position(row_number, col_number), player))
    }

    fn check_diagonals(&self, player: &Player) -> bool {
        (self.matches(&Position(1, 1), player) && self.matches(&Position(2, 2), player) && self.matches(&Position(3, 3), player))
            || (self.matches(&Position(3, 1), player)
                && self.matches(&Position(2, 2), player)
                && self.matches(&Position(1, 3), player))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("=================");
        self.squares.iter().for_each(|row| {
            output.push_str(&format!(
                "\n  {}  |  {}  |  {}  \n=================",
                &row[0],
                &row[1],
                &row[2]
            ));
        });
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_move_on_empty_board() {
        let position = Position(2, 2);
        let mut expected_board = Board::default();
        expected_board.set_position(&position, &Player::X);

        let mut board = Board::default();
        let result = board.play_move(&Player::X, position);

        assert_eq!(result, Result::Ok(GameState::InProgress));
        assert_eq!(board, expected_board);
    }

    #[test]
    fn play_move_on_occupied_square() {
        let position = Position(2, 2);

        let mut board = Board::default();
        board.set_position(&position, &Player::O);
        let result = board.play_move(&Player::X, position);

        assert_eq!(
            result,
            Result::Err(GameError {
                message: format!(
                "Player {} has already played in this square. Please select a different square.",
                Player::O
            )
            })
        );
    }

    #[test]
    fn check_if_player_won_on_row() {
        for row_number in 1..=3 {
            let mut board = Board::default();
            (1..=3).for_each(|col_number| {
                board.set_position(
                    &Position(row_number, col_number),
                    &Player::O,
                )
            });

            assert!(board.check_if_player_won(&Player::O))
        }
    }

    #[test]
    fn check_if_player_won_on_col() {
        for col_number in 1..=3 {
            let mut board = Board::default();
            (1..=3).for_each(|row_number| {
                board.set_position(
                    &Position(row_number, col_number),
                    &Player::O,
                )
            });

            assert!(board.check_if_player_won(&Player::O))
        }
    }

    #[test]
    fn check_if_player_won_on_diagonal() {
        let mut board = Board::default();
        board.set_position(&Position(1, 1), &Player::O);
        board.set_position(&Position(2, 2), &Player::O);
        board.set_position(&Position(3, 3), &Player::O);

        assert!(board.check_if_player_won(&Player::O));

        board = Board::default();
        board.set_position(&Position(1, 3), &Player::O);
        board.set_position(&Position(2, 2), &Player::O);
        board.set_position(&Position(3, 1), &Player::O);

        assert!(board.check_if_player_won(&Player::O));
    }

    #[test]
    fn check_if_player_won_on_in_progress_board() {
        let mut board = Board::default();
        board.set_position(&Position(1, 1), &Player::O);
        board.set_position(&Position(2, 2), &Player::O);
        board.set_position(&Position(3, 2), &Player::O);

        assert!(!board.check_if_player_won(&Player::O));
    }
}
