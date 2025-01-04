use std::fmt::Display;

use super::{player::Player, position::Position, GameError, GameState};

#[derive(PartialEq, Debug)]
pub struct Board {
    pub squares: Vec<Vec<Option<Player>>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: (0..3)
                .map(|_| (0..3).map(|_| Option::None).collect())
                .collect(),
        }
    }

    pub fn play_move(
        &mut self,
        player: &Player,
        position: Position,
    ) -> Result<GameState, GameError> {
        match self.get_position(&position) {
            Some(filled) => {
                Result::Err(GameError {
                    message: format!("Player {} has already played in this square. Please select a different square.", filled),
                })
            },
            None => {
                self.set_position(&position, player);

                if self.check_if_player_won(player) {
                    Result::Ok(GameState::Winner(player.clone()))
                } else if self.board_full() {
                    Result::Ok(GameState::NoWinner)
                } else {
                    Result::Ok(GameState::InProgress)
                }
            }
        }
    }

    pub fn check_if_player_won(&self, player: &Player) -> bool {
        (0..3).any(|row_number| self.check_row(player, row_number))
            || (0..3).any(|col_number| self.check_col(player, col_number))
            || self.check_diagonals(player)
    }

    fn get_position(&self, position: &Position) -> Option<&Player> {
        self.squares[(position.row as usize) - 1][(position.col as usize) - 1].as_ref()
    }

    fn set_position(&mut self, position: &Position, player: &Player) {
        self.squares[(position.row as usize) - 1][(position.col as usize) - 1] =
            Some(player.clone());
    }

    fn board_full(&self) -> bool {
        self.squares
            .iter()
            .all(|row| row.iter().all(|square| square.is_some()))
    }

    fn matches(&self, row: usize, col: usize, player: &Player) -> bool {
        match &self.squares[row][col] {
            Some(p) => p == player,
            None => false,
        }
    }

    fn check_col(&self, player: &Player, col_number: usize) -> bool {
        (0..3).all(|row_number| self.matches(row_number, col_number, player))
    }

    fn check_row(&self, player: &Player, row_number: usize) -> bool {
        (0..3).all(|col_number| self.matches(row_number, col_number, player))
    }

    fn check_diagonals(&self, player: &Player) -> bool {
        (self.matches(0, 0, player) && self.matches(1, 1, player) && self.matches(2, 2, player))
            || (self.matches(2, 0, player)
                && self.matches(1, 1, player)
                && self.matches(0, 2, player))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_player(player: &Option<Player>) -> String {
            match player {
                Some(p) => format!("{}", p),
                None => String::from(" "),
            }
        }

        let mut output = String::from("=================");
        self.squares.iter().for_each(|row| {
            output.push_str(&format!(
                "\n  {}  |  {}  |  {}  \n=================",
                format_player(&row[0]),
                format_player(&row[1]),
                format_player(&row[2])
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
        let position = Position { row: 2, col: 2 };
        let mut expected_board = Board::new();
        expected_board.set_position(&position, &Player::X);

        let mut board = Board::new();
        let result = board.play_move(&Player::X, position);

        assert_eq!(result, Result::Ok(GameState::InProgress));
        assert_eq!(board, expected_board);
    }

    #[test]
    fn play_move_on_occupied_square() {
        let position = Position { row: 2, col: 2 };

        let mut board = Board::new();
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
            let mut board = Board::new();
            (1..=3).for_each(|col_number| {
                board.set_position(
                    &Position {
                        row: row_number,
                        col: col_number,
                    },
                    &Player::O,
                )
            });

            assert!(board.check_if_player_won(&Player::O))
        }
    }

    #[test]
    fn check_if_player_won_on_col() {
        for col_number in 1..=3 {
            let mut board = Board::new();
            (1..=3).for_each(|row_number| {
                board.set_position(
                    &Position {
                        row: row_number,
                        col: col_number,
                    },
                    &Player::O,
                )
            });

            assert!(board.check_if_player_won(&Player::O))
        }
    }

    #[test]
    fn check_if_player_won_on_diagonal() {
        let mut board = Board::new();
        board.set_position(&Position { row: 1, col: 1 }, &Player::O);
        board.set_position(&Position { row: 2, col: 2 }, &Player::O);
        board.set_position(&Position { row: 3, col: 3 }, &Player::O);

        assert!(board.check_if_player_won(&Player::O));

        board = Board::new();
        board.set_position(&Position { row: 1, col: 3 }, &Player::O);
        board.set_position(&Position { row: 2, col: 2 }, &Player::O);
        board.set_position(&Position { row: 3, col: 1 }, &Player::O);

        assert!(board.check_if_player_won(&Player::O));
    }

    #[test]
    fn check_if_player_won_on_in_progress_board() {
        let mut board = Board::new();
        board.set_position(&Position { row: 1, col: 1 }, &Player::O);
        board.set_position(&Position { row: 2, col: 2 }, &Player::O);
        board.set_position(&Position { row: 3, col: 2 }, &Player::O);

        assert!(!board.check_if_player_won(&Player::O));
    }
}
