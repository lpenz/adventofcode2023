// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
use rayon::prelude::*;

pub const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn line(input: &str) -> IResult<&str, Vec<i64>> {
        let (input, nums) = multi::separated_list1(character::space1, character::i64)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, nums))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<i64>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

pub fn diffs(nums: &[i64]) -> Vec<i64> {
    nums.par_windows(2).map(|l| l[1] - l[0]).collect()
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 3);
    Ok(())
}
