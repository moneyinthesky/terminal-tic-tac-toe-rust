use std::fmt::Display;

use console::style;

#[derive(PartialEq, Debug, Clone)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn from_move(move_number: i32) -> Player {
        if move_number % 2 == 1 {
            Player::X
        } else {
            Player::O
        }
    }

    pub fn styled(&self) -> console::StyledObject<String> {
        match self {
            Player::X => style(format!("{self}")).bold().green(),
            Player::O => style(format!("{self}")).bold().red(),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_moves_should_be_played_by_x() {
        assert_eq!(Player::from_move(3), Player::X)
    }

    #[test]
    fn even_moves_should_be_played_by_o() {
        assert_eq!(Player::from_move(4), Player::O)
    }
}
