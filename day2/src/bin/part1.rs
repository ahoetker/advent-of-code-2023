use anyhow::{ensure, Result};
use day2::{
    game::{Dice, Game},
    parse::parse_game,
};
use itertools::Itertools;
use std::fs::read_to_string;

pub fn validate(game: Game, max_dice: &Dice) -> Result<Game> {
    let Dice { red, green, blue } = game.most_dice_shown();
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
    Ok(game)
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
                .filter_map(|(_, game)| validate(game, &max_dice).ok())
                .map(|game| game.number)
                .sum()
        })?;
    println!("{possible_sum}");
    Ok(())
}
