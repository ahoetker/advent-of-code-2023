use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct Calibration {
    first_digit: char,
    second_digit: char,
}

impl Calibration {
    pub fn parse(text: &str) -> Result<Self> {
        let digits = parse_digits_from_text(text.to_string());
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

fn parse_digits_from_text(text: String) -> Vec<char> {
    let mut digits = Vec::new();
    let mut text = text;
    while let Some(c) = text.chars().next() {
        if c.is_ascii_digit() {
            digits.push(c);
            let _ = text.remove(0);
            continue;
        }
        if text.len() >= 3 {
            if &text[..3] == "one" {
                digits.push('1');
                for _ in [0; 2] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..3] == "two" {
                digits.push('2');
                for _ in [0; 2] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..3] == "six" {
                digits.push('6');
                for _ in [0; 2] {
                    let _ = text.remove(0);
                }
                continue;
            }
        }
        if text.len() >= 4 {
            if &text[..4] == "zero" {
                digits.push('0');
                for _ in [0; 3] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..4] == "four" {
                digits.push('4');
                for _ in [0; 3] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..4] == "five" {
                digits.push('5');
                for _ in [0; 3] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..4] == "nine" {
                digits.push('9');
                for _ in [0; 3] {
                    let _ = text.remove(0);
                }
                continue;
            }
        }
        if text.len() >= 5 {
            if &text[..5] == "three" {
                digits.push('3');
                for _ in [0; 4] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..5] == "seven" {
                digits.push('7');
                for _ in [0; 4] {
                    let _ = text.remove(0);
                }
                continue;
            }
            if &text[..5] == "eight" {
                digits.push('8');
                for _ in [0; 4] {
                    let _ = text.remove(0);
                }
                continue;
            }
        }
        let _ = text.remove(0);
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
