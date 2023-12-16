// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day09::*;

use rayon::prelude::*;

fn calc_prev(nums: &[i64]) -> i64 {
    let mut mult = -1;
    let mut result = nums[0];
    let mut currdiff = diffs(nums);
    while !currdiff.par_iter().all(|n| n == &0) {
        result += mult * currdiff[0];
        mult *= -1;
        currdiff = diffs(&currdiff);
    }
    result
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    Ok(input.par_iter().map(|v| calc_prev(v)).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
