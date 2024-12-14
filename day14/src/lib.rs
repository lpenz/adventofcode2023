// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use sqrid::postrait::PosT;

use std::fmt;

pub const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Rock,
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            'O' => Ok(Cell::Rock),
            other => Err(eyre!("invalid cell {}", other)),
        }
    }
}

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(100, 100, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Rock => write!(f, "O"),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#O")(input)?;
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
    assert_eq!(input.len(), 10);
    assert_eq!(input[0].len(), 10);
    Ok(())
}

pub fn tilt(size: usize, mut grid: Grid, dir: Dir) -> Grid {
    let qas = if dir == Dir::N || dir == Dir::W {
        Pos::iter().collect::<Vec<_>>()
    } else {
        Pos::iter().rev().collect::<Vec<_>>()
    };
    for qa_rock in qas {
        if grid[qa_rock] != Cell::Rock {
            continue;
        }
        let mut pos = qa_rock;
        while let Ok(qa_new) = pos + dir {
            let t = qa_new.tuple();
            let t = (t.0 as usize, t.1 as usize);
            if t.0 >= size || t.1 >= size || grid[qa_new] != Cell::Empty {
                break;
            }
            pos = qa_new;
        }
        if pos != qa_rock {
            grid[qa_rock] = Cell::Empty;
            grid[pos] = Cell::Rock;
        }
    }
    grid
}

pub fn grid_load(size: usize, grid: &Grid) -> usize {
    Pos::iter()
        .map(|pos| {
            if grid[pos] == Cell::Rock {
                let t = pos.tuple();
                size - t.1 as usize
            } else {
                0
            }
        })
        .sum()
}
