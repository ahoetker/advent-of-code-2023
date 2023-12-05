use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till, take_while},
    character::complete::{digit1, multispace0, newline},
    combinator::opt,
    multi::many1,
    sequence::{delimited, terminated},
    IResult,
};
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
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

#[derive(Debug, PartialEq)]
struct Game {
    number: u32,
    draws: Vec<Dice>,
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
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Dice {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add for Dice {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl<'a> Sum<&'a Dice> for Dice {
    fn sum<I>(iter: I) -> Dice
    where
        I: Iterator<Item = &'a Dice>,
    {
        iter.fold(
            Dice {
                red: 0,
                green: 0,
                blue: 0,
            },
            |a, b| a + *b,
        )
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

fn game_number(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, number) = take_till(|c| c == ':')(input)?;
    Ok((input, number.parse::<u32>().unwrap()))
}

fn quantity_color(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, quantity) = delimited(multispace0, digit1, multispace0)(input)?;
    let quantity = quantity.parse::<u32>().unwrap();
    let (input, color) = terminated(take_while(char::is_alphabetic), opt(tag(",")))(input)?;
    let color = Color::parse(color).unwrap();
    Ok((input, (quantity, color)))
}

fn quantity_color_multiple(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
    many1(quantity_color)(input)
}

fn parse_draw(input: &str) -> IResult<&str, Dice> {
    let (input, quantity_colors) = terminated(quantity_color_multiple, opt(tag(";")))(input)?;
    Ok((input, Dice::from(quantity_colors)))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, number) = game_number(input)?;
    let (remains, draws) = delimited(tag(":"), many1(parse_draw), opt(newline))(input)?;
    Ok((remains, Game { number, draws }))
}

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/part1.txt")?;
    let max_dice = Dice {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_sum: u32 = input
        .lines()
        .map(|line| parse_game(line).map_err(|err| err.to_owned()))
        .process_results(|games| {
            games
                .filter_map(|(_, game)| game.validate(&max_dice).ok())
                .map(|game| game.number)
                .sum()
        })?;
    println!("{possible_sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("Game 1: 3 Blue, 4 Red;", 1, ": 3 Blue, 4 Red;")]
    #[case(
        "Game 2: 1 Blue, 2 Green; 3 Green, 4 Blue;",
        2,
        ": 1 Blue, 2 Green; 3 Green, 4 Blue;"
    )]
    fn get_game_number(
        #[case] input: &str,
        #[case] expected_number: u32,
        #[case] expected_remains: &str,
    ) {
        let (remains, number) = game_number(input).unwrap();
        assert_eq!(number, expected_number);
        assert_eq!(remains, expected_remains);
    }

    #[rstest]
    #[case("3 blue,", 3, Color::Blue)]
    #[case("2 green", 2, Color::Green)]
    #[case(" 1 red", 1, Color::Red)]
    fn test_quantity_color(
        #[case] input: &str,
        #[case] expected_quantity: u32,
        #[case] expected_color: Color,
    ) {
        let (remains, (quantity, color)) = quantity_color(input).unwrap();
        assert_eq!(quantity, expected_quantity);
        assert_eq!(color, expected_color);
        assert_eq!(remains, "");
    }

    #[rstest]
    #[case("3 red, 2 blue, 1 green", vec![(3, Color::Red), (2, Color::Blue), (1, Color::Green)])]
    fn test_quantity_color_multiple(#[case] input: &str, #[case] expected: Vec<(u32, Color)>) {
        let (remains, quantity_colors) = quantity_color_multiple(input).unwrap();
        assert_eq!(quantity_colors, expected);
        assert_eq!(remains, "");
    }

    #[rstest]
    #[case("3 red, 2 blue, 1 green", Dice { red: 3, blue: 2, green: 1})]
    fn test_parse_draw(#[case] input: &str, #[case] expected: Dice) {
        let (remains, draw) = parse_draw(input).unwrap();
        assert_eq!(draw, expected);
        assert_eq!(remains, "");
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game { number: 1, draws: vec![Dice { red: 4, blue: 3, green: 0}, Dice { red: 1, green: 2, blue: 6}, Dice {red: 0, green: 2, blue: 0}]})]
    fn test_parse_game(#[case] input: &str, #[case] expected_game: Game) {
        let (remains, game) = parse_game(input).unwrap();
        assert_eq!(game, expected_game);
        assert_eq!(remains, "");
    }
}
