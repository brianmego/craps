use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Roll {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}
impl From<u8> for Roll {
    fn from(value: u8) -> Self {
        match value {
            2 => Roll::Two,
            3 => Roll::Three,
            4 => Roll::Four,
            5 => Roll::Five,
            6 => Roll::Six,
            7 => Roll::Seven,
            8 => Roll::Eight,
            9 => Roll::Nine,
            10 => Roll::Ten,
            11 => Roll::Eleven,
            12 => Roll::Twelve,
            _ => unreachable!(),
        }
    }
}
impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numerical = match self {
            Roll::Two => 2,
            Roll::Three => 3,
            Roll::Four => 4,
            Roll::Five => 5,
            Roll::Six => 6,
            Roll::Seven => 7,
            Roll::Eight => 8,
            Roll::Nine => 9,
            Roll::Ten => 10,
            Roll::Eleven => 11,
            Roll::Twelve => 12,
        };
        f.write_str(&numerical.to_string())
    }
}
