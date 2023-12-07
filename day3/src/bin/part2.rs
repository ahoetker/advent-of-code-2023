use anyhow::Result;
use day3::{Schematic, SchematicNumber};
use std::fs::read_to_string;
use std::str::FromStr;

fn is_adjacent(schematic_number: SchematicNumber, gear_row: usize, gear_col: usize) -> bool {
    let SchematicNumber {
        row, col, length, ..
    } = schematic_number;
    gear_row >= row.saturating_sub(1)
        && gear_row <= row.saturating_add(1)
        && gear_col >= col.saturating_sub(1)
        && gear_col <= col.saturating_add(length)
}

fn sum_gear_ratios(schematic: &Schematic) -> u32 {
    schematic
        .symbols
        .iter()
        .filter_map(|((row, col), symbol)| {
            if symbol == &'*' {
                let adjacent_numbers: Vec<&SchematicNumber> = schematic
                    .numbers
                    .iter()
                    .filter(|number| is_adjacent(**number, *row, *col))
                    .collect();
                if adjacent_numbers.len() == 2 {
                    return Some(adjacent_numbers[0].value * adjacent_numbers[1].value);
                }
            }
            None
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/input.txt")?;
    let schematic = Schematic::from_str(&input)?;
    let gear_ratios_sum = sum_gear_ratios(&schematic);
    println!("{gear_ratios_sum}");
    Ok(())
}
