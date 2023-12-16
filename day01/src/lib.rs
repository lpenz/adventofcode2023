// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

pub const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

pub mod parser {
    pub use aoc::parser::*;

    // use super::*;

    fn line(input: &str) -> IResult<&str, String> {
        let (input, chars) = character::alphanumeric1(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, chars.to_string()))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<String>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE1.as_bytes())?.len(), 4);
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?.len(), 7);
    Ok(())
}
