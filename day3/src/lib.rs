use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{digit1, satisfy},
    error::Error,
    multi::many1,
    Finish, IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<SchematicNumber>,
    pub symbols: HashMap<(usize, usize), char>,
}

#[derive(Debug, Copy, Clone)]
pub struct SchematicNumber {
    pub value: u32,
    pub row: usize,
    pub col: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum SchematicEntry<'a> {
    Dots(&'a str),
    Symbol(char),
    Number(&'a str),
}

impl FromStr for Schematic {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_schematic(s).finish() {
            Ok((_remaining, schematic)) => Ok(schematic),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn parse_schematic(s: &str) -> IResult<&str, Schematic> {
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
                        value: number.parse::<_>().unwrap(),
                        row: line_number,
                        col: col_number,
                        length: number.len(),
                    });
                    col_number += number.len();
                }
            }
        }
    }
    Ok(("", Schematic { numbers, symbols }))
}

fn schematic_row(s: &str) -> IResult<&str, Vec<SchematicEntry>> {
    many1(alt((schematic_dots, schematic_number, schematic_symbol)))(s)
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
