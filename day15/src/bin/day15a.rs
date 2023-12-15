// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

// pub use color_eyre::Result;
use std::io::{stdin, BufRead};

use day15::*;

fn hash(steps: &str) -> u32 {
    steps.chars().fold(0, |value, c| {
        let ascii = c as u32;
        ((value + ascii) * 17) % 256
    })
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    Ok(input.into_iter().map(|step| hash(&step)).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1320);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}