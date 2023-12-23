// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::collections::HashSet;

pub const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Galaxy,
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cell::Galaxy),
            '.' => Ok(Cell::Empty),
            other => Err(eyre!("invalid cell {}", other)),
        }
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#")(input)?;
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
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 10);
    Ok(())
}

pub fn calc_distances(inc: i64, input: Vec<Vec<Cell>>) -> Result<i64> {
    let galaxies = input
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().filter_map(move |(x, cell)| {
                if cell == Cell::Galaxy {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(i64, i64)>>();
    let xs = galaxies
        .iter()
        .map(|(x, _)| x)
        .copied()
        .collect::<HashSet<_>>();
    let xmax = xs.iter().max().copied().unwrap();
    let ys = galaxies
        .iter()
        .map(|(_, y)| y)
        .copied()
        .collect::<HashSet<_>>();
    let ymax = ys.iter().max().copied().unwrap();
    let mut galaxies2 = Vec::<(i64, i64)>::new();
    let mut yinc = 0_i64;
    for y in 0..=ymax {
        if !ys.contains(&y) {
            yinc += inc;
        } else {
            let mut xinc = 0_i64;
            for x in 0..=xmax {
                if !xs.contains(&x) {
                    xinc += inc;
                } else if galaxies.contains(&(x, y)) {
                    galaxies2.push((x + xinc, y + yinc));
                }
            }
        }
    }
    let gpairs = galaxies2
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| galaxies2[i + 1..].iter().map(move |g2| (g1, g2)))
        .collect::<Vec<_>>();
    Ok(gpairs
        .into_iter()
        .map(|(g1, g2)| (g2.0 - g1.0).abs() + (g2.1 - g1.1).abs())
        .sum())
}
