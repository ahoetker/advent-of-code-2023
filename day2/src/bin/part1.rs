use anyhow::Result;
use day2::{game::Dice, parse::parse_game};
use itertools::Itertools;
use std::fs::read_to_string;

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
