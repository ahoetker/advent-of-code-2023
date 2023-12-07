use anyhow::Result;
use day3::{Schematic, SchematicNumber};
use std::fs::read_to_string;
use std::str::FromStr;

fn sum_part_numbers(schematic: &Schematic) -> u32 {
    schematic
        .numbers
        .iter()
        .filter_map(
            |SchematicNumber {
                 row,
                 col,
                 value,
                 length,
             }| {
                for row_number in row.saturating_sub(1)..=row.saturating_add(1) {
                    for col_number in col.saturating_sub(1)..=col.saturating_add(*length) {
                        if schematic.symbols.contains_key(&(row_number, col_number)) {
                            return Some(value);
                        }
                    }
                }
                None
            },
        )
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/input.txt")?;
    let schematic = Schematic::from_str(&input)?;
    let part_numbers_sum = sum_part_numbers(&schematic);
    println!("{part_numbers_sum}");
    Ok(())
}
