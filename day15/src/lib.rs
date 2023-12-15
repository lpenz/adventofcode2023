// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

pub const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

pub mod parser {
    use aoc::parser::*;

    // use super::*;

    fn step(input: &str) -> IResult<&str, String> {
        let (input, chars) = multi::many1(character::none_of(",\n"))(input)?;
        Ok((input, chars.into_iter().collect::<String>()))
    }

    fn line(input: &str) -> IResult<&str, Vec<String>> {
        let (input, steps) = multi::separated_list1(bytes::tag(","), step)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, steps))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<String>> {
        let parsed: Result<Vec<Vec<String>>> = aoc::parse_with!(multi::many1(line), bufin);
        Ok(parsed?.pop().unwrap())
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 11);
    Ok(())
}
