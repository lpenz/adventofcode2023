// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::fmt;

pub const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[derive(Debug, Default, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Digit(char),
    Symbol(char),
}

impl Cell {
    pub fn digit(&self) -> Result<char> {
        match self {
            Cell::Empty => Err(eyre!("empty cell")),
            Cell::Symbol(_) => Err(eyre!("symbol cell")),
            Cell::Digit(i) => Ok(*i),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Digit(i) => write!(f, "{}", i),
            Cell::Symbol(c) => write!(f, "{}", c),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn empty(input: &str) -> IResult<&str, Cell> {
        let (input, _) = bytes::tag(".")(input)?;
        Ok((input, Cell::Empty))
    }

    fn digit(input: &str) -> IResult<&str, Cell> {
        let (input, digit) = character::one_of("0123456789")(input)?;
        Ok((input, Cell::Digit(digit)))
    }

    fn symbol(input: &str) -> IResult<&str, Cell> {
        let (input, symbol) = character::one_of("#*+$&%=@/-")(input)?;
        Ok((input, Cell::Symbol(symbol)))
    }

    fn cell(input: &str) -> IResult<&str, Cell> {
        branch::alt((empty, branch::alt((digit, symbol))))(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, line) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, line))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 10);
    Ok(())
}
