use anyhow::{anyhow, ensure, Result};

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
    pub fn validate(self, max_dice: &Dice) -> Result<Self> {
        let Dice { red, green, blue } = self.most_dice_shown();
        ensure!(
            red <= max_dice.red,
            format!(
                "{} red dice were shown at once, but only {} are possible!",
                red, max_dice.red
            )
        );
        ensure!(
            green <= max_dice.green,
            format!(
                "{} green dice were shown at once, but only {} are possible!",
                green, max_dice.green
            )
        );
        ensure!(
            blue <= max_dice.blue,
            format!(
                "{} blue dice were shown at once, but only {} are possible!",
                blue, max_dice.blue
            )
        );
        Ok(self)
    }

    fn most_dice_shown(&self) -> Dice {
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

    pub fn new(number: u32, draws: Vec<Dice>) -> Self {
        Self { number, draws }
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
