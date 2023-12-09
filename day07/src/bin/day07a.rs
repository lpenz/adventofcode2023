// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use itertools::Itertools;
use std::io::{stdin, BufRead};

use day07::*;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|(hand, bid)| (hand.value(), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as i64 + 1) * bid)
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 6440);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
