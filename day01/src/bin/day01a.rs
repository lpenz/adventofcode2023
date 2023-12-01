// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use color_eyre::eyre;
use std::io::{stdin, BufRead};

use aoc::parser::*;

use day01::*;

fn firstnum(input: &str) -> IResult<&str, u32> {
    let (input, _) = character::alpha0(input)?;
    let (input, numstr) = character::one_of("0123456789")(input)?;
    let (input, _) = character::alphanumeric0(input)?;
    Ok((input, numstr.to_digit(10).unwrap()))
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let lines = parser::parse(bufin)?;
    Ok(lines
        .into_iter()
        .map(|line| {
            let first = all_consuming(firstnum)(&line)
                .finish()
                .map_err(|e| eyre!("error reading first digit {:?}", e))?
                .1;
            let reversed = line.chars().rev().collect::<String>();
            let last = all_consuming(firstnum)(&reversed)
                .finish()
                .map_err(|e| eyre!("error reading last digit {:?}", e))?
                .1;
            Ok(first * 10 + last)
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 142);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
