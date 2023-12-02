// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
pub use std::collections::BTreeMap;

pub const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub type Set = BTreeMap<Color, u32>;
pub type Game = Vec<Set>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn red(input: &str) -> IResult<&str, Color> {
        let (input, _) = bytes::tag("red")(input)?;
        Ok((input, Color::Red))
    }

    fn green(input: &str) -> IResult<&str, Color> {
        let (input, _) = bytes::tag("green")(input)?;
        Ok((input, Color::Green))
    }

    fn blue(input: &str) -> IResult<&str, Color> {
        let (input, _) = bytes::tag("blue")(input)?;
        Ok((input, Color::Blue))
    }

    fn color(input: &str) -> IResult<&str, Color> {
        branch::alt((red, branch::alt((green, blue))))(input)
    }

    fn entry(input: &str) -> IResult<&str, (Color, u32)> {
        let (input, num) = character::u32(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, c) = color(input)?;
        Ok((input, (c, num)))
    }

    fn set(input: &str) -> IResult<&str, Set> {
        let (input, setlist) = multi::separated_list1(bytes::tag(", "), entry)(input)?;
        let set = setlist.into_iter().collect::<Set>();
        Ok((input, set))
    }

    fn game(input: &str) -> IResult<&str, Game> {
        let (input, _) = bytes::tag("Game ")(input)?;
        let (input, _) = character::u32(input)?;
        let (input, _) = bytes::tag(": ")(input)?;
        let (input, sets) = multi::separated_list1(bytes::tag("; "), set)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, sets))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Game>> {
        aoc::parse_with!(multi::many1(game), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 5);
    Ok(())
}
