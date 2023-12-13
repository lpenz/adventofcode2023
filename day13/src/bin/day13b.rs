// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;
use std::io::{stdin, BufRead};

use day13::*;

pub fn flip(g: &mut HashSet<Xy>, xy: &Xy) {
    if g.contains(xy) {
        g.remove(xy);
    } else {
        g.insert(*xy);
    }
}

fn calc_summary(vecgrid: VecGrid) -> usize {
    let (mut grid, size) = vecgrid2hashset(&vecgrid);
    let old = find_mirror_summary(&grid, &size, None);
    for y_smudge in 0..size.1 {
        for x_smudge in 0..size.0 {
            let smudge = (x_smudge, y_smudge);
            flip(&mut grid, &smudge);
            if let Some(summary) = find_mirror_summary(&grid, &size, old) {
                return summary;
            }
            flip(&mut grid, &smudge);
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
    assert_eq!(process(EXAMPLE.as_bytes())?, 400);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
