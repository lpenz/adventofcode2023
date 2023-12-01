// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

pub const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

pub mod parser {
    use aoc::parser::*;

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
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 4);
    Ok(())
}
