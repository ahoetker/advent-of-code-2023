use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug)]
struct Calibration {
    first_digit: char,
    second_digit: char,
}

impl Calibration {
    pub fn parse(text: &str) -> Result<Self> {
        let first_digit = text
            .chars()
            .find(|c| c.is_ascii_digit())
            .context(format!("No digits found in {text}"))?;
        let second_digit = text
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .context(format!("No digits found in {text}"))?;
        Ok(Self {
            first_digit,
            second_digit,
        })
    }

    pub fn value(&self) -> u32 {
        format!("{}{}", self.first_digit, self.second_digit)
            .parse()
            .unwrap()
    }
}

fn main() -> Result<()> {
    let text = read_to_string("puzzle_inputs/input.txt")?;
    let calibration_sum: u32 = text
        .trim_end()
        .split('\n')
        .map(Calibration::parse)
        .process_results(|calibrations| {
            calibrations.map(|calibration| calibration.value()).sum()
        })?;
    println!("{calibration_sum}");
    Ok(())
}
