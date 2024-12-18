// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

use sqrid::postrait::PosT;
pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(140, 140, true);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub fn grid_get_number(grid: &mut Grid, qa_digit: Pos) -> Result<u32> {
    if !matches!(grid[qa_digit], Cell::Digit(_)) {
        return Err(eyre!(
            "cell {:?} with {} doesn't have a digit",
            qa_digit,
            grid[qa_digit]
        ));
    }
    // Found a digit, go left:
    let qa_start = std::iter::successors(Some(qa_digit), |pos| {
        (*pos + Dir::W)
            .ok()
            .filter(|pos| matches!(grid[pos], Cell::Digit(_)))
    })
    .last()
    .ok_or_else(|| eyre!("could not find first digit from {:?}", qa_digit))?;
    // We are at the start of the number, pick up the digits
    let number_str = Pos::iter_range(qa_start, Pos::BOTTOM_RIGHT)
        .map_while(|pos| grid[pos].digit().ok())
        .collect::<String>();
    // Empty the used cells:
    for pos in Pos::iter_range(qa_start, Pos::BOTTOM_RIGHT) {
        if grid[pos].digit().is_err() {
            break;
        }
        grid[pos] = Cell::Empty;
    }
    number_str
        .parse::<u32>()
        .map_err(|_| eyre!("invalid number {}", number_str))
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
