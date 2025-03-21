mod play_areas;
mod roll;

use std::fmt::Display;

use play_areas::*;
use roll::Roll;

fn main() {
    let mut table: Table = Table::default();
    for _ in 0..100 {
        table.roll();
    }
}

fn roll_dice() -> Roll {
    let die1 = rand::random_range(1..=6);
    let die2 = rand::random_range(1..=6);
    (die1 + die2).into()
}

type Point = Roll;
enum Button {
    On(Point),
    Off,
}
impl Button {
    fn turn_on(&self, roll: Roll) -> Button {
        Button::On(roll)
    }
    fn turn_off(&self) -> Button {
        Button::Off
    }
}

struct Table {
    playspots: Vec<TableLoc>,
    button: Button,
}
impl Table {
    fn turn_button_on(&mut self, roll: Roll) {
        self.button = self.button.turn_on(roll)
    }
    fn turn_button_off(&mut self) {
        self.button = self.button.turn_off()
    }
    fn roll(&mut self) {
        let roll = roll_dice();
        println!("Roll: {}", &roll);
        for i in self.playspots.iter() {
            let inner = i.inner();
            match inner.handle_roll(&roll, &self.button) {
                HandleRollResult::Win(payout) => {
                    println!("{} Wins! Payout: {}", inner.name(), payout)
                }
                HandleRollResult::Lose => println!("{} Loses!", inner.name()),
                _ => {}
            }
        }
        match &self.button {
            Button::On(point) => {
                if &roll == point || roll == Roll::Seven {
                    self.turn_button_off();
                    println!("Button is OFF");
                }
                if roll == Roll::Seven {
                    println!("Bad luck! Change shooters");
                    println!();
                }
            }
            Button::Off => {
                if [
                    Roll::Four,
                    Roll::Five,
                    Roll::Six,
                    Roll::Eight,
                    Roll::Nine,
                    Roll::Ten,
                ]
                .contains(&roll)
                {
                    println!("Button is placed on {}", roll);
                    self.turn_button_on(roll);
                }
            }
        }
    }
}
impl Default for Table {
    fn default() -> Self {
        Self {
            playspots: vec![
                TableLoc::PassLine,
                TableLoc::DontPass,
                TableLoc::Field,
                TableLoc::BigSix,
            ],
            button: Button::Off,
        }
    }
}

enum TableLoc {
    PassLine,
    DontPass,
    Field,
    BigSix,
}
impl TableLoc {
    fn inner(&self) -> Box<dyn BettingRule> {
        match self {
            TableLoc::PassLine => Box::new(PassLine),
            TableLoc::DontPass => Box::new(DontPass),
            TableLoc::Field => Box::new(Field),
            TableLoc::BigSix => Box::new(BigSix),
        }
    }
}

struct Payout {
    numerator: u8,
    denominator: u8,
}
impl Display for Payout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Payout: {} to {}",
            self.numerator, self.denominator
        ))
    }
}

impl Payout {
    fn new(numerator: u8, denominator: u8) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

enum HandleRollResult {
    Win(Payout),
    Lose,
    NoEffect,
}
