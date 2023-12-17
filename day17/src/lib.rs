// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

pub const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

pub type Cell = u32;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, val) = character::one_of("0123456789")(input)?;
        Ok((input, val.to_digit(10).unwrap()))
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
    assert_eq!(input.len(), 13);
    assert_eq!(input[0].len(), 13);
    Ok(())
}

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(141, 141, false);
// pub type Sqrid = sqrid::sqrid_create!(13, 13, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, u32);

pub type Griddir = sqrid::grid_create!(Sqrid, String);

pub fn path_debug(_size: u16, gheat: &Grid, path: &[Dir]) {
    let mut gheatacum = Grid::default();
    let mut pos = Pos::TOP_LEFT;
    let mut heat = 0;
    let mut gdir = Griddir::default();
    for dir in path {
        gdir[pos] = dir.name_utf8().to_string();
        pos = (pos + dir).unwrap();
        heat += gheat[pos];
        gheatacum[pos] = heat;
    }
    eprintln!("{:1}", gdir);
    eprintln!("{:>4}", gheatacum);
}

pub type Heat = u32;

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct State {
    pub pos: Pos,
    pub lastdir: Option<Dir>,
    pub dircount: usize,
}

pub fn solve<F: Fn(&State, Dir) -> bool>(size: u16, gheat: Grid, dir_valid: F) -> Result<u32> {
    let mut frontier = BinaryHeap::<(Reverse<Heat>, State)>::default();
    frontier.push((Reverse(0), State::default()));
    let mut visited = HashSet::<State>::default();
    let mut heatacummap = HashMap::<State, Heat>::default();
    heatacummap.insert(State::default(), 0);
    let goal = Pos::try_from((size - 1, size - 1)).unwrap();
    while let Some((_priority, st)) = frontier.pop() {
        let pos = st.pos;
        if visited.contains(&st) {
            continue;
        }
        let heatacum = heatacummap[&st];
        if pos == goal {
            return Ok(heatacum);
        }
        for dir in Dir::iter::<false>() {
            if !dir_valid(&st, dir) {
                continue;
            }
            let Ok(newpos) = pos + dir else { continue };
            let t = newpos.tuple();
            if t.0 >= size || t.1 >= size {
                continue;
            }
            let heatacum = heatacum + gheat[newpos];
            let dircount = if Some(dir) == st.lastdir {
                st.dircount + 1
            } else {
                1
            };
            let newst = State {
                pos: newpos,
                lastdir: Some(dir),
                dircount,
            };
            let e = heatacummap.entry(newst).or_insert(heatacum);
            if heatacum < *e {
                *e = heatacum;
            }
            if visited.contains(&newst) {
                continue;
            }
            let dist = Pos::manhattan(&newpos, &goal) as u32;
            let priority = Reverse(heatacum + dist);
            frontier.push((priority, newst));
        }
        visited.insert(st);
    }
    unreachable!();
}
