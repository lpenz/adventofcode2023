// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day13::*;

fn calc_summary(vecgrid: VecGrid) -> usize {
    let (grid, size) = vecgrid2hashset(&vecgrid);
    if let Some(summary) = find_mirror_summary(&grid, &size, None) {
        return summary;
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
