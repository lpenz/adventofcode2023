// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day06::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (times, distances) = input;
    let races = std::iter::zip(times, distances).collect::<Vec<_>>();
    Ok(races
        .into_iter()
        .map(|(time, distance)| {
            (0..=time)
                .map(|button_time| button_time * (time - button_time))
                .filter(|i| i > &distance)
                .count()
        })
        .product())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 288);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
