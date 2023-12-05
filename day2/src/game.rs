use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub number: u32,
    pub draws: Vec<Dice>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Dice {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Game {
    pub fn most_dice_shown(&self) -> Dice {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        self.draws.iter().for_each(|draw| {
            if draw.red > red {
                red = draw.red;
            }
            if draw.green > green {
                green = draw.green;
            }
            if draw.blue > blue {
                blue = draw.blue;
            }
        });
        Dice { red, green, blue }
    }
}

impl From<Vec<(u32, Color)>> for Dice {
    fn from(quantity_colors: Vec<(u32, Color)>) -> Self {
        let red = match quantity_colors
            .iter()
            .find(|(_, color)| *color == Color::Red)
        {
            Some((quantity, _)) => *quantity,
            None => 0,
        };
        let green = match quantity_colors
            .iter()
            .find(|(_, color)| *color == Color::Green)
        {
            Some((quantity, _)) => *quantity,
            None => 0,
        };
        let blue = match quantity_colors
            .iter()
            .find(|(_, color)| *color == Color::Blue)
        {
            Some((quantity, _)) => *quantity,
            None => 0,
        };
        Self { red, green, blue }
    }
}

impl Color {
    pub fn parse(input: &str) -> Result<Color> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(anyhow!("Cannot parse {input} as Color.")),
        }
    }
}
