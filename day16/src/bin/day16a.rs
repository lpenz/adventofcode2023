// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day16::*;

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(110, 110, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub type Beam = (Pos, Dir);

pub struct Beams(pub HashSet<Beam>);

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

fn process(size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input.clone())?;
    let mut beams = Beams([(Pos::TOP_LEFT, Dir::E)].into_iter().collect());
    beams.process(&grid);
    let mut energized = [Pos::TOP_LEFT].into_iter().collect::<HashSet<Pos>>();
    let mut unchanged = 0;
    while !beams.0.is_empty() && unchanged < size {
        let old = energized.clone();
        beams.next(size, &grid);
        energized.extend(beams.0.iter().map(|(pos, _)| pos));
        if energized != old {
            unchanged = 0;
        } else {
            unchanged += 1;
        }
    }
    Ok(energized.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 46);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(110, stdin().lock())?);
    Ok(())
}
