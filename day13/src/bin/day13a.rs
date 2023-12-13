// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day13::*;

pub type Xy = (usize, usize);

fn vecgrid2hashset(vecgrid: &VecGrid) -> (HashSet<Xy>, Xy) {
    let size = (vecgrid[0].len(), vecgrid.len());
    (
        (0..size.1)
            .flat_map(|y| (0..size.0).filter_map(move |x| vecgrid[y][x].then_some((x, y))))
            .collect::<HashSet<Xy>>(),
        size,
    )
}

fn gridmatch_x(mirror: usize, size: &Xy, g: &HashSet<Xy>) -> bool {
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

fn gridmatch_y(mirror: usize, size: &Xy, g: &HashSet<Xy>) -> bool {
    let ymin = if 2 * mirror >= size.1 + 1 {
        2 * mirror - size.1 + 2
    } else {
        0
    };
    for y in ymin..(mirror + 1) {
        let ymirror = 2 * mirror + 1 - y;
        if ymirror >= size.1 {
            continue;
        }
        for x in 0..size.0 {
            if g.contains(&(x, y)) != g.contains(&(x, ymirror)) {
                return false;
            }
        }
    }
    true
}

fn calc_summary(vecgrid: VecGrid) -> usize {
    let (grid, size) = vecgrid2hashset(&vecgrid);
    let max = std::cmp::max(size.0, size.1);
    for mirror in (0..max - 1).rev() {
        if mirror < size.0 - 1 && gridmatch_x(mirror, &size, &grid) {
            return mirror + 1;
        }
        if mirror < size.1 - 1 && gridmatch_y(mirror, &size, &grid) {
            return 100 * (mirror + 1);
        }
    }
    panic!("no mirror found");
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input.into_iter().map(calc_summary).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 405);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
