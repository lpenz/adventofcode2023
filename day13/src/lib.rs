// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use std::collections::HashSet;

pub const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

pub type Xy = (usize, usize);
pub type VecGrid = Vec<Vec<bool>>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, bool> {
        let (input, c) = character::one_of(".#")(input)?;
        Ok((input, c == '#'))
    }

    fn line(input: &str) -> IResult<&str, Vec<bool>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    fn grid(input: &str) -> IResult<&str, VecGrid> {
        let (input, g) = multi::many1(line)(input)?;
        Ok((input, g))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<VecGrid>> {
        aoc::parse_with!(multi::separated_list1(character::newline, grid), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 2);
    assert_eq!(input[0].len(), 7);
    assert_eq!(input[0][1].len(), 9);
    assert_eq!(input[1].len(), 7);
    assert_eq!(input[1][1].len(), 9);
    Ok(())
}

pub fn vecgrid2hashset(vecgrid: &VecGrid) -> (HashSet<Xy>, Xy) {
    let size = (vecgrid[0].len(), vecgrid.len());
    (
        (0..size.1)
            .flat_map(|y| (0..size.0).filter_map(move |x| vecgrid[y][x].then_some((x, y))))
            .collect::<HashSet<Xy>>(),
        size,
    )
}

pub fn translate(g: &HashSet<Xy>) -> HashSet<Xy> {
    g.iter().map(|(x, y)| (*y, *x)).collect()
}

fn gridmatch(mirror: usize, size: &Xy, g: &HashSet<Xy>) -> bool {
    let xmin = if 2 * mirror >= size.0 + 1 {
        2 * mirror - size.0 + 2
    } else {
        0
    };
    for x in xmin..(mirror + 1) {
        let xmirror = 2 * mirror + 1 - x;
        if xmirror >= size.0 {
            continue;
        }
        for y in 0..size.1 {
            if g.contains(&(x, y)) != g.contains(&(xmirror, y)) {
                return false;
            }
        }
    }
    true
}

pub fn find_mirror_summary(grid: &HashSet<Xy>, size: &Xy, old: Option<usize>) -> Option<usize> {
    let (gridy, sizey) = (translate(grid), (size.1, size.0));
    let max = std::cmp::max(size.0, size.1);
    for mirror in (0..max - 1).rev() {
        if mirror < size.0 - 1 && Some(mirror + 1) != old && gridmatch(mirror, size, grid) {
            return Some(mirror + 1);
        }
        if mirror < size.1 - 1
            && Some(100 * (mirror + 1)) != old
            && gridmatch(mirror, &sizey, &gridy)
        {
            return Some(100 * (mirror + 1));
        }
    }
    None
}

pub fn print_grid(grid: &HashSet<Xy>, size: Xy) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let xy = (x, y);
            eprint!("{}", if grid.contains(&xy) { '#' } else { '.' });
        }
        eprintln!();
    }
}
