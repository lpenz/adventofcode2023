// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;
use std::io::{stdin, BufRead};

use day09::*;

fn diffs(nums: &[i64]) -> Vec<i64> {
    nums.par_windows(2).map(|l| l[1] - l[0]).collect()
}

fn calc_next(nums: &Vec<i64>) -> i64 {
    let mut lastsum = nums[nums.len() - 1];
    let mut currdiff = diffs(nums);
    while !currdiff.par_iter().all(|n| n == &0) {
        lastsum += currdiff[currdiff.len() - 1];
        currdiff = diffs(&currdiff);
    }
    lastsum
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    Ok(input.par_iter().map(calc_next).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 114);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
