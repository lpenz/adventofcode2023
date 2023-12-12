// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day12::*;

pub fn unfold_calc_arrangements(row: &Row, record: &[u32]) -> usize {
    let mut bigrow = row.clone();
    let mut bigrecord = record.to_vec();
    for _ in 0..4 {
        bigrow.0.push(Cell::Unknown);
        bigrow.0.extend(row.0.iter());
        bigrecord.extend(record.iter());
    }
    calc_arrangements(&bigrow, &bigrecord)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|entry| unfold_calc_arrangements(&entry.0, &entry.1))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 525152);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
