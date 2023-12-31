// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

pub const EXAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

pub const EXAMPLE3: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

pub const EXAMPLE4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Ground,
    Start,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Cell {
    pub fn is_curve(&self) -> bool {
        matches!(self, Cell::NE | Cell::NW | Cell::SW | Cell::SE)
    }
}

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(140, 140, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub fn next_qr(grid: &Grid, pos: Pos, dir: Dir) -> Option<Dir> {
    match (grid[pos], dir) {
        (Cell::NS, Dir::N) => Some(Dir::N),
        (Cell::NS, Dir::S) => Some(Dir::S),
        (Cell::EW, Dir::E) => Some(Dir::E),
        (Cell::EW, Dir::W) => Some(Dir::W),
        (Cell::NE, Dir::W) => Some(Dir::N),
        (Cell::NE, Dir::S) => Some(Dir::E),
        (Cell::NW, Dir::E) => Some(Dir::N),
        (Cell::NW, Dir::S) => Some(Dir::W),
        (Cell::SW, Dir::E) => Some(Dir::S),
        (Cell::SW, Dir::N) => Some(Dir::W),
        (Cell::SE, Dir::W) => Some(Dir::S),
        (Cell::SE, Dir::N) => Some(Dir::E),
        _ => None,
    }
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Cell::NS),
            '-' => Ok(Cell::EW),
            'L' => Ok(Cell::NE),
            'J' => Ok(Cell::NW),
            '7' => Ok(Cell::SW),
            'F' => Ok(Cell::SE),
            '.' => Ok(Cell::Ground),
            'S' => Ok(Cell::Start),
            other => Err(eyre!("invalid pipe {}", other)),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of("|-LJ7F.S")(input)?;
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
    assert_eq!(parser::parse(EXAMPLE1.as_bytes())?.len(), 5);
    assert_eq!(parser::parse(EXAMPLE1.as_bytes())?[0].len(), 5);
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?.len(), 5);
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?[0].len(), 5);
    Ok(())
}
