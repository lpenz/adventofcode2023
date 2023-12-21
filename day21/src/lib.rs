// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Rock,
    Start,
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Rock),
            'S' => Ok(Cell::Start),
            other => Err(eyre!("invalid cell {}", other)),
        }
    }
}

impl From<Cell> for char {
    fn from(c: Cell) -> Self {
        match c {
            Cell::Empty => '.',
            Cell::Rock => '#',
            Cell::Start => 'S',
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(r".#S")(input)?;
        Ok((input, Cell::try_from(c).unwrap()))
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 11);
    Ok(())
}
