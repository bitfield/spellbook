use std::{num::ParseIntError, str::FromStr};

#[expect(clippy::exhaustive_enums, reason = "the only possibilities")]
#[derive(Debug)]
pub enum NumberKind {
    Negative(i32),
    NotANumber,
    Positive(i32),
}

impl FromStr for NumberKind {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match i32::from_str(input) {
            Ok(num) if num < 0 => NumberKind::Negative(num),
            Ok(num) => NumberKind::Positive(num),
            Err(_) => NumberKind::NotANumber,
        })
    }
}

fn main() {
    for input in ["bacon", "12", "-9", "0"] {
        println!(r#""{}" -> {:?}"#, input, input.parse::<NumberKind>());
    }
    // "bacon" -> Ok(NotANumber)
    // "12" -> Ok(Positive(12))
    // "-9" -> Ok(Negative(-9))
    // "0" -> Ok(Positive(0))
}
