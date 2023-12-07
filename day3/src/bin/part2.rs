use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{digit1, satisfy},
    multi::many1,
    IResult,
};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Schematic<'a> {
    pub numbers: Vec<SchematicNumber<'a>>,
    pub symbols: HashMap<(usize, usize), char>,
}

#[derive(Debug, Copy, Clone)]
struct SchematicNumber<'a> {
    pub number: &'a str,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum SchematicEntry<'a> {
    Dots(&'a str),
    Symbol(char),
    Number(&'a str),
}

fn schematic_dots(s: &str) -> IResult<&str, SchematicEntry> {
    let (s, dots) = take_while1(|c| c == '.')(s)?;
    Ok((s, SchematicEntry::Dots(dots)))
}

fn schematic_symbol(s: &str) -> IResult<&str, SchematicEntry> {
    let (s, symbol) = satisfy(|c| !(c.is_ascii_digit() || c == '.'))(s)?;
    Ok((s, SchematicEntry::Symbol(symbol)))
}

fn schematic_number(s: &str) -> IResult<&str, SchematicEntry> {
    let (s, number) = digit1(s)?;
    Ok((s, SchematicEntry::Number(number)))
}

fn schematic_row(s: &str) -> IResult<&str, Vec<SchematicEntry>> {
    many1(alt((schematic_dots, schematic_number, schematic_symbol)))(s)
}

fn parse_schematic(s: &str) -> IResult<&str, Schematic> {
    let mut numbers: Vec<SchematicNumber> = vec![];
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();

    for (line_number, line) in s.lines().enumerate() {
        let mut col_number = 0;
        let (_, row) = schematic_row(line)?;
        for entry in row {
            match entry {
                SchematicEntry::Dots(dots) => {
                    col_number += dots.len();
                }
                SchematicEntry::Symbol(symbol) => {
                    symbols.insert((line_number, col_number), symbol);
                    col_number += 1;
                }
                SchematicEntry::Number(number) => {
                    numbers.push(SchematicNumber {
                        number,
                        row: line_number,
                        col: col_number,
                    });
                    col_number += number.len();
                }
            }
        }
    }
    Ok(("", Schematic { numbers, symbols }))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1", SchematicEntry::Number("1"), "")]
    #[case("301..", SchematicEntry::Number("301"), "..")]
    fn test_schematic_number(
        #[case] s: &str,
        #[case] expected: SchematicEntry,
        #[case] expected_remains: &str,
    ) {
        let (remains, entry) = schematic_number(s).unwrap();
        assert_eq!(entry, expected);
        assert_eq!(remains, expected_remains);
    }

    #[rstest]
    #[case(".", SchematicEntry::Dots("."), "")]
    #[case("...", SchematicEntry::Dots("..."), "")]
    #[case("..1%..", SchematicEntry::Dots(".."), "1%..")]
    fn test_schematic_dots(
        #[case] s: &str,
        #[case] expected: SchematicEntry,
        #[case] expected_remains: &str,
    ) {
        let (remains, entry) = schematic_dots(s).unwrap();
        assert_eq!(entry, expected);
        assert_eq!(remains, expected_remains);
    }

    #[rstest]
    #[case("%", SchematicEntry::Symbol('%'), "")]
    #[case("?.", SchematicEntry::Symbol('?'), ".")]
    #[case("*123", SchematicEntry::Symbol('*'), "123")]
    fn test_schematic_symbol(
        #[case] s: &str,
        #[case] expected: SchematicEntry,
        #[case] expected_remains: &str,
    ) {
        let (remains, entry) = schematic_symbol(s).unwrap();
        assert_eq!(entry, expected);
        assert_eq!(remains, expected_remains);
    }

    #[rstest]
    #[case(".....+.58.", vec![SchematicEntry::Dots("....."), SchematicEntry::Symbol('+'), SchematicEntry::Dots("."), SchematicEntry::Number("58"), SchematicEntry::Dots(".")], "")]
    fn test_schematic_row(
        #[case] s: &str,
        #[case] expected: Vec<SchematicEntry>,
        #[case] expected_remains: &str,
    ) {
        let (remains, row) = schematic_row(s).unwrap();
        assert_eq!(row, expected);
        assert_eq!(remains, expected_remains);
    }
}
