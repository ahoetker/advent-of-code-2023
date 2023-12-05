use anyhow::Result;
use day2::{game::Dice, parse::parse_game};
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/part1.txt")?;
    let sum_of_powers: u32 = input
        .lines()
        .map(|line| parse_game(line).map_err(|err| err.to_owned()))
        .process_results(|games| {
            games
                .map(|(_, game)| {
                    let Dice { red, green, blue } = game.most_dice_shown();
                    red * green * blue
                })
                .sum()
        })?;
    println!("{sum_of_powers}");
    Ok(())
}
