use anyhow::Result;
use day7::Hand;
use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn process(input_file: &str) -> Result<u32> {
    let input = read_to_string(input_file)?;
    let mut hands: Vec<Hand> = input.lines().map(Hand::from_str).try_collect()?;
    hands.sort();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum::<u32>())
}

fn main() -> Result<()> {
    let result = process("puzzle_inputs/input.txt")?;
    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let result = process("puzzle_inputs/sample.txt")?;
        assert_eq!(result, 6440);
        Ok(())
    }
}
