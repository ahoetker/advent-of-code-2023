use anyhow::Result;
use day3::{parse_schematic, Schematic, SchematicNumber};
use std::fs::read_to_string;

fn is_adjacent(schematic_number: SchematicNumber, gear_row: usize, gear_col: usize) -> bool {
    let SchematicNumber { row, col, number } = schematic_number;
    gear_row >= row.saturating_sub(1)
        && gear_row <= row.saturating_add(1)
        && gear_col >= col.saturating_sub(1)
        && gear_col <= col.saturating_add(number.len())
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
                    return Some(
                        adjacent_numbers[0].number.parse::<u32>().unwrap()
                            * adjacent_numbers[1].number.parse::<u32>().unwrap(),
                    );
                }
            }
            None
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/input.txt")?;
    let (_, schematic) = parse_schematic(&input).map_err(|e| e.to_owned())?;
    let gear_ratios_sum = sum_gear_ratios(&schematic);
    println!("{gear_ratios_sum}");
    Ok(())
}
