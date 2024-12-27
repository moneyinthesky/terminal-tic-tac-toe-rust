use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts = s.split(",").collect::<Vec<_>>();

        if parts.len() == 2 {
            let row_part: Result<i8, _> = parts[0].trim().parse();
            let col_part: Result<i8, _> = parts[1].trim().parse();

            match (row_part, col_part) {
                (Ok(row), Ok(col)) => {
                    if (1..=3).contains(&row) && (1..=3).contains(&col) {
                        Ok(Position { row, col })
                    } else {
                        Err(String::from(
                            "ROW and COL values should be numbers between 1 and 3",
                        ))
                    }
                }
                _ => Err(String::from(
                    "ROW and COL values should be numbers between 1 and 3",
                )),
            }
        } else {
            Err(String::from("Select square in format \"ROW,COL\""))
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.row, self.col)
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Position::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_from_correctly_formatted_string() {
        assert_eq!(
            Position::from_str("1,2"),
            Result::Ok(Position { row: 1, col: 2 })
        );
    }

    #[test]
    fn position_from_correctly_formatted_string_with_spaces() {
        assert_eq!(
            Position::from_str(" 3 , 2 "),
            Result::Ok(Position { row: 3, col: 2 })
        );
    }

    #[test]
    fn position_from_incorrectly_formatted_string() {
        assert_eq!(
            Position::from_str(",1,2"),
            Result::Err(String::from("Select square in format \"ROW,COL\""))
        );
    }

    #[test]
    fn position_from_empty_string() {
        assert_eq!(
            Position::from_str(""),
            Result::Err(String::from("Select square in format \"ROW,COL\""))
        );
    }

    #[test]
    fn position_with_out_of_range_values() {
        assert_eq!(
            Position::from_str("9,2"),
            Result::Err(String::from("ROW and COL values should be numbers between 1 and 3"))
        );

        assert_eq!(
            Position::from_str("2,9"),
            Result::Err(String::from("ROW and COL values should be numbers between 1 and 3"))
        );
    }

    #[test]
    fn position_with_large_values() {
        assert_eq!(
            Position::from_str("128,2"),
            Result::Err(String::from("ROW and COL values should be numbers between 1 and 3"))
        );
    }

    #[test]
    fn position_from_non_numeric_value() {
        assert_eq!(
            Position::from_str("abc,2"),
            Result::Err(String::from("ROW and COL values should be numbers between 1 and 3"))
        );
    }
}
