use anyhow::Result;
use day3::{parse_schematic, Schematic, SchematicNumber};
use std::fs::read_to_string;

fn sum_part_numbers(schematic: &Schematic) -> u32 {
    schematic
        .numbers
        .iter()
        .filter_map(|SchematicNumber { row, col, number }| {
            for row_number in row.saturating_sub(1)..=row.saturating_add(1) {
                for col_number in col.saturating_sub(1)..=col.saturating_add(number.len()) {
                    if schematic.symbols.contains_key(&(row_number, col_number)) {
                        return Some(number.parse::<u32>().unwrap());
                    }
                }
            }
            None
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("puzzle_inputs/input.txt")?;
    let (_, schematic) = parse_schematic(&input).map_err(|e| e.to_owned())?;
    let part_numbers_sum = sum_part_numbers(&schematic);
    println!("{part_numbers_sum}");
    Ok(())
}
