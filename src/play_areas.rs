use crate::{Button, HandleRollResult, Payout, roll::Roll};

pub trait BettingRule {
    fn handle_roll(&self, roll: &Roll, button: &Button) -> HandleRollResult;
    fn name(&self) -> String;
}

pub struct PassLine;
pub struct DontPass;
pub struct Field;
pub struct BigSix;

impl BettingRule for PassLine {
    fn handle_roll(&self, roll: &Roll, button: &Button) -> HandleRollResult {
        match button {
            Button::On(point) => match roll {
                Roll::Seven => HandleRollResult::Lose,
                r => match r == point {
                    true => HandleRollResult::Win(Payout::new(1, 1)),
                    false => HandleRollResult::NoEffect,
                },
            },
            Button::Off => match roll {
                Roll::Seven | Roll::Eleven => HandleRollResult::Win(Payout::new(1, 1)),
                Roll::Two | Roll::Three | Roll::Twelve => HandleRollResult::Lose,
                _ => HandleRollResult::NoEffect,
            },
        }
    }

    fn name(&self) -> String {
        "Pass Line".into()
    }
}

impl BettingRule for DontPass {
    fn handle_roll(&self, roll: &Roll, button: &Button) -> HandleRollResult {
        match button {
            Button::On(point) => match roll {
                Roll::Seven => HandleRollResult::Win(Payout::new(1, 1)),
                r => match r == point {
                    true => HandleRollResult::Lose,
                    false => HandleRollResult::NoEffect,
                },
            },
            Button::Off => match roll {
                Roll::Seven | Roll::Eleven => HandleRollResult::Lose,
                Roll::Two | Roll::Three => HandleRollResult::Win(Payout::new(1, 1)),
                _ => HandleRollResult::NoEffect,
            },
        }
    }

    fn name(&self) -> String {
        "Don't Pass".into()
    }
}

impl BettingRule for Field {
    fn handle_roll(&self, roll: &Roll, _: &Button) -> HandleRollResult {
        match roll {
            Roll::Three | Roll::Four | Roll::Nine | Roll::Ten | Roll::Eleven => {
                HandleRollResult::Win(Payout::new(1, 1))
            }
            Roll::Two => HandleRollResult::Win(Payout::new(2, 1)),
            Roll::Twelve => HandleRollResult::Win(Payout::new(3, 1)),
            _ => HandleRollResult::Lose,
        }
    }

    fn name(&self) -> String {
        "Field".into()
    }
}

impl BettingRule for BigSix {
    fn handle_roll(&self, roll: &Roll, _: &Button) -> HandleRollResult {
        match roll {
            Roll::Six => HandleRollResult::Win(Payout::new(1, 1)),
            Roll::Seven => HandleRollResult::Lose,
            _ => HandleRollResult::NoEffect,
        }
    }

    fn name(&self) -> String {
        "Big Six".into()
    }
}
