// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Report, Result};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;

pub const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    MirrorU,
    MirrorD,
    SplitH,
    SplitV,
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '\\' => Ok(Cell::MirrorU),
            '/' => Ok(Cell::MirrorD),
            '-' => Ok(Cell::SplitH),
            '|' => Ok(Cell::SplitV),
            other => Err(eyre!("invalid pipe {}", other)),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(r".\/-|")(input)?;
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

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(110, 110, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub type Beam = (Pos, Dir);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Beams(pub BTreeSet<Beam>);

impl Beams {
    pub fn next(&mut self, size: u16, grid: &Grid) {
        let beams = std::mem::take(&mut self.0);
        for beam in beams {
            let (pos, dir) = beam;
            if let Ok(newpos) = pos + dir {
                let t = newpos.tuple();
                if t.0 >= size || t.1 >= size {
                    continue;
                }
                self.0.insert((newpos, dir));
            }
        }
        self.process(grid);
    }

    pub fn process(&mut self, grid: &Grid) {
        let beams = std::mem::take(&mut self.0);
        for beam in beams {
            let (pos, dir) = beam;
            match (grid[pos], dir) {
                (Cell::MirrorU, Dir::N) => {
                    self.0.insert((pos, Dir::W));
                }
                (Cell::MirrorU, Dir::E) => {
                    self.0.insert((pos, Dir::S));
                }
                (Cell::MirrorU, Dir::S) => {
                    self.0.insert((pos, Dir::E));
                }
                (Cell::MirrorU, Dir::W) => {
                    self.0.insert((pos, Dir::N));
                }
                (Cell::MirrorD, Dir::N) => {
                    self.0.insert((pos, Dir::E));
                }
                (Cell::MirrorD, Dir::E) => {
                    self.0.insert((pos, Dir::N));
                }
                (Cell::MirrorD, Dir::S) => {
                    self.0.insert((pos, Dir::W));
                }
                (Cell::MirrorD, Dir::W) => {
                    self.0.insert((pos, Dir::S));
                }
                (Cell::SplitH, Dir::N) => {
                    self.0.insert((pos, Dir::E));
                    self.0.insert((pos, Dir::W));
                }
                (Cell::SplitH, Dir::S) => {
                    self.0.insert((pos, Dir::E));
                    self.0.insert((pos, Dir::W));
                }
                (Cell::SplitV, Dir::E) => {
                    self.0.insert((pos, Dir::N));
                    self.0.insert((pos, Dir::S));
                }
                (Cell::SplitV, Dir::W) => {
                    self.0.insert((pos, Dir::N));
                    self.0.insert((pos, Dir::S));
                }
                (_, _) => {
                    self.0.insert((pos, dir));
                }
            }
        }
    }
}

impl fmt::Display for Beams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            let t = b.0.tuple();
            write!(f, "({},{};{})", t.0, t.1, b.1)?;
        }
        Ok(())
    }
}

pub type Energized = BTreeSet<Pos>;
pub type Cache = HashMap<Beams, Energized>;

pub fn calc_energized(size: u16, grid: &Grid, cache: &mut Cache, start: Beam) -> usize {
    let mut beams = Beams([start].into_iter().collect());
    beams.process(grid);
    let mut energized = Energized::default();
    energized.insert(start.0);
    let mut unchanged = 0;
    let mut path = vec![beams.clone()];
    let mut ecache = Energized::default();
    while !beams.0.is_empty() && unchanged < size {
        let old = energized.clone();
        beams.next(size, grid);
        if let Some(e) = cache.get(&beams) {
            energized.extend(e.iter());
            ecache = e.clone();
            break;
        }
        path.push(beams.clone());
        energized.extend(beams.0.iter().map(|(pos, _)| pos));
        if energized != old {
            unchanged = 0;
        } else {
            unchanged += 1;
        }
    }
    for p in path.into_iter().rev() {
        ecache.extend(p.0.iter().map(|(pos, _)| pos));
        cache.insert(p, ecache.clone());
    }
    energized.len()
}
