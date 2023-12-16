// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn all_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
        let (input, _) = bytes::tag("Time:")(input)?;
        let (input, _) = multi::many1(bytes::tag(" "))(input)?;
        let (input, times) =
            multi::separated_list1(multi::many1(bytes::tag(" ")), character::u32)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, _) = bytes::tag("Distance:")(input)?;
        let (input, _) = multi::many1(bytes::tag(" "))(input)?;
        let (input, distances) =
            multi::separated_list1(multi::many1(bytes::tag(" ")), character::u32)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (times, distances)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<u32>, Vec<u32>)> {
        aoc::parse_with!(all_input, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let example = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(example.0.len(), 3);
    assert_eq!(example.1.len(), 3);
    Ok(())
}
