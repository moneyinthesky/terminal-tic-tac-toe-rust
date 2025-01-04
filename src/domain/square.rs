use std::fmt::Display;

use super::player::Player;

#[derive(PartialEq, Debug, Clone, Default)]
pub enum Square {
    X,
    O,
    #[default]
    EMPTY
}

impl Square {
    pub fn from_player(player: &Player) -> Square {
        match player {
            Player::X => Square::X,
            Player::O => Square::O,
        }
    }

    pub fn filled_by(&self, player: &Player) -> bool {
        match player {
            Player::X => self == &Square::X,
            Player::O => self == &Square::O,
        }
    }

    pub fn is_filled(&self) -> bool {
        self != &Square::EMPTY
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
            Square::EMPTY => write!(f, " "),
        }
    }
}