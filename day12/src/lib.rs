// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Report, Result};
use std::fmt;
use std::str::FromStr;

pub const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Cell {
    Ok,
    Broken,
    Unknown,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Ok => write!(f, "."),
            Cell::Broken => write!(f, "#"),
            Cell::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Ok),
            '#' => Ok(Cell::Broken),
            '?' => Ok(Cell::Unknown),
            other => Err(eyre!("invalid spring {}", other)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Row(pub Vec<Cell>);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.0 {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

impl FromStr for Row {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row(s
            .chars()
            .map(Cell::try_from)
            .collect::<Result<Vec<Cell>>>()?))
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#?")(input)?;
        Ok((input, Cell::try_from(c).unwrap()))
    }

    fn line(input: &str) -> IResult<&str, (Row, Vec<u32>)> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::space1(input)?;
        let (input, runs) = multi::separated_list1(bytes::tag(","), character::u32)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (Row(cells), runs)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Row, Vec<u32>)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 6);
    Ok(())
}
