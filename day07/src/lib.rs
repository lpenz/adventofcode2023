// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Report, Result};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

pub const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(pub char);

impl Card {
    pub fn value(&self, joker: bool) -> i64 {
        match self {
            Card('A') => 13,
            Card('K') => 12,
            Card('Q') => 11,
            Card('J') => {
                if joker {
                    0
                } else {
                    10
                }
            }
            Card('T') => 9,
            Card(n) => n.to_digit(10).expect("invalid card") as i64 - 1,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Type {
    Highest = 0,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hand(pub [Card; 5]);

impl Hand {
    pub fn htype(&self) -> Type {
        let card_count = self
            .0
            .iter()
            .fold(BTreeMap::<Card, usize>::new(), |mut counts, card| {
                let e = counts.entry(*card).or_default();
                *e += 1;
                counts
            });
        let count_card = card_count
            .into_iter()
            .map(|(card, count)| (count, card))
            .sorted()
            .rev()
            .collect::<Vec<_>>();
        if count_card[0].0 == 5 {
            Type::FiveOfKind
        } else if count_card[0].0 == 4 {
            Type::FourOfKind
        } else if count_card[0].0 == 3 && count_card[1].0 == 2 {
            Type::FullHouse
        } else if count_card[0].0 == 3 {
            Type::ThreeOfKind
        } else if count_card[0].0 == 2 && count_card[1].0 == 2 {
            Type::TwoPair
        } else if count_card[0].0 == 2 {
            Type::OnePair
        } else {
            Type::Highest
        }
    }

    pub fn iter_jokers(&self) -> impl Iterator<Item = Hand> + '_ {
        std::iter::once(*self)
            .chain(
                self.0
                    .iter()
                    .filter(|&c| c != &Card('J'))
                    .unique()
                    .map(|c| {
                        let mut hand = *self;
                        for pos in hand.0.iter_mut() {
                            if *pos == Card('J') {
                                *pos = *c;
                            }
                        }
                        hand
                    }),
            )
            .unique()
    }

    pub fn best_type(&self) -> Type {
        self.iter_jokers()
            .map(|hand| hand.htype())
            .max_by_key(|&t| t as i64)
            .unwrap()
    }

    pub fn value(&self, joker: bool) -> i64 {
        let cards_value = self.cards_value(joker);
        if joker {
            self.best_type() as i64 * 14_i64.pow(6) + cards_value
        } else {
            self.htype() as i64 * 14_i64.pow(6) + cards_value
        }
    }

    pub fn cards_value(&self, joker: bool) -> i64 {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| c.value(joker) * 14_i64.pow(i as u32))
            .sum()
    }
}

impl TryFrom<&[Card]> for Hand {
    type Error = Report;
    fn try_from(cards: &[Card]) -> Result<Self, Self::Error> {
        if cards.len() != 5 {
            return Err(eyre!("invalid length for hand"));
        }
        Ok(Hand([cards[0], cards[1], cards[2], cards[3], cards[4]]))
    }
}

impl FromStr for Hand {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().map(Card).collect::<Vec<_>>();
        Hand::try_from(cards.as_ref())
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0 {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[test]
fn test_iter_jokers() -> Result<()> {
    assert_eq!(
        Hand::from_str("AAAAA")?.iter_jokers().collect::<Vec<_>>(),
        vec![Hand::from_str("AAAAA")?]
    );
    assert_eq!(
        Hand::from_str("JAAAA")?.iter_jokers().collect::<Vec<_>>(),
        vec![Hand::from_str("JAAAA")?, Hand::from_str("AAAAA")?,]
    );
    assert_eq!(
        Hand::from_str("JJAAA")?.iter_jokers().collect::<Vec<_>>(),
        vec![Hand::from_str("JJAAA")?, Hand::from_str("AAAAA")?,]
    );
    Ok(())
}

#[test]
fn test_hand_type() -> Result<()> {
    assert_eq!(Hand::from_str("AAAAA")?.htype(), Type::FiveOfKind);
    assert_eq!(Hand::from_str("AA1AA")?.htype(), Type::FourOfKind);
    assert_eq!(Hand::from_str("A2A2A")?.htype(), Type::FullHouse);
    assert_eq!(Hand::from_str("A2A1A")?.htype(), Type::ThreeOfKind);
    assert_eq!(Hand::from_str("A221A")?.htype(), Type::TwoPair);
    assert_eq!(Hand::from_str("A2219")?.htype(), Type::OnePair);
    assert_eq!(Hand::from_str("A2319")?.htype(), Type::Highest);
    Ok(())
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn card(input: &str) -> IResult<&str, Card> {
        let (input, c) = character::one_of("AKQJT98765432")(input)?;
        Ok((input, Card(c)))
    }

    fn line(input: &str) -> IResult<&str, (Hand, i64)> {
        let (input, cards) = multi::count(card, 5)(input)?;
        let (input, _) = character::space1(input)?;
        let (input, bid) = character::i64(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (Hand::try_from(cards.as_ref()).unwrap(), bid)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Hand, i64)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 5);
    Ok(())
}
