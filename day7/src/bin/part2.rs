use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, digit1, multispace0, newline},
    combinator::opt,
    error::Error,
    sequence::delimited,
    Finish, IResult,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'J' => Ok(Self::Joker),
            _ => Err(anyhow!("{} is not a valid Camel Card.", c)),
        }
    }
}

impl Card {
    pub fn strength(&self) -> u32 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u32,
    pub hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .strength()
            .cmp(&other.hand_type.strength())
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hand(s).finish() {
            Ok((_remaining, hand)) => Ok(hand),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_hand(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = hand_cards(s)?;
    let (s, bid) = hand_bid(s)?;
    let hand = Hand::new(cards, bid);
    Ok((s, hand))
}

fn hand_cards(s: &str) -> IResult<&str, Vec<Card>> {
    let (s, cards) = alphanumeric1(s)?;
    let cards = cards.chars().map(Card::try_from).try_collect().unwrap();
    Ok((s, cards))
}

fn hand_bid(s: &str) -> IResult<&str, u32> {
    let (s, bid) = delimited(multispace0, digit1, opt(newline))(s)?;
    let bid = bid.parse::<_>().unwrap();
    Ok((s, bid))
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u32) -> Self {
        let possible_cards = get_possible_cards(&cards);
        let best_hand_type = possible_cards
            .iter()
            .map(|c| determine_hand_type(c))
            .max()
            .unwrap();
        Self {
            cards,
            bid,
            hand_type: best_hand_type,
        }
    }
}

fn get_possible_cards(cards: &[Card]) -> Vec<Vec<Card>> {
    Card::iter()
        .filter(|&substitution_card| Card::Joker != substitution_card)
        .map(|substitution_card| {
            cards
                .iter()
                .map(|&card| {
                    if Card::Joker == card {
                        substitution_card
                    } else {
                        card
                    }
                })
                .collect()
        })
        .collect()
}

fn determine_hand_type(cards: &[Card]) -> HandType {
    let mut cards_by_type: HashMap<u32, usize> = HashMap::new();
    cards.iter().for_each(|card| {
        cards_by_type
            .entry(card.strength())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });
    if cards_by_type.values().any(|&v| v == 5) {
        return HandType::FiveOfAKind;
    }
    if cards_by_type.values().any(|&v| v == 4) {
        return HandType::FourOfAKind;
    }
    if cards_by_type.values().any(|&v| v == 3) {
        if cards_by_type.values().any(|&v| v == 2) {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    }
    match cards_by_type.values().filter(|&&v| v == 2).count() {
        2 => return HandType::TwoPair,
        1 => return HandType::OnePair,
        _ => (),
    }
    HandType::HighCard
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandType {
    pub fn strength(&self) -> u32 {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

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
        assert_eq!(result, 5905);
        Ok(())
    }
}
