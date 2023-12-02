use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::read_to_string;

lazy_static! {
    static ref SPELLED_DIGITS: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("zero", '0');
        m.insert("one", '1');
        m.insert("two", '2');
        m.insert("three", '3');
        m.insert("four", '4');
        m.insert("five", '5');
        m.insert("six", '6');
        m.insert("seven", '7');
        m.insert("eight", '8');
        m.insert("nine", '9');
        m
    };
}

#[derive(Debug, Copy, Clone)]
struct Calibration {
    first_digit: char,
    second_digit: char,
}

impl Calibration {
    pub fn parse(text: &str) -> Result<Self> {
        let digits = parse_digits_from_text(text);
        if digits.is_empty() {
            return Err(anyhow!("No digits parsed from: {}", text));
        }
        Ok(Self {
            first_digit: digits[0],
            second_digit: digits[digits.len() - 1],
        })
    }

    pub fn value(&self) -> u32 {
        format!("{}{}", self.first_digit, self.second_digit)
            .parse()
            .unwrap()
    }
}

fn parse_digits_from_text(text: &str) -> Vec<char> {
    let mut digits = Vec::new();
    let mut i = 0;
    let length = text.len();
    'outer: while i < length {
        let c = text.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            digits.push(c);
            i += 1;
            continue;
        }
        for spelled_length in 3..=5 {
            if length - i >= spelled_length {
                if let Some(digit) = SPELLED_DIGITS.get(&text[i..i + spelled_length]) {
                    digits.push(*digit);
                    i += spelled_length - 1;
                    continue 'outer;
                }
            }
        }
        i += 1;
    }
    digits
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
