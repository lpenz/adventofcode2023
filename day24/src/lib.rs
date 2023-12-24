// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

pub type Xyz = (f64, f64, f64);
pub type Stone = (Xyz, Xyz, usize);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn xyz(input: &str) -> IResult<&str, Xyz> {
        let (input, x) = character::i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = character::space1(input)?;
        let (input, y) = character::i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = character::space1(input)?;
        let (input, z) = character::i64(input)?;
        Ok((input, (x as f64, y as f64, z as f64)))
    }

    fn line(input: &str) -> IResult<&str, Stone> {
        let (input, pos) = xyz(input)?;
        let (input, _) = tag(" @")(input)?;
        let (input, _) = character::space1(input)?;
        let (input, vel) = xyz(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (pos, vel, 0)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Stone>> {
        let input: Result<Vec<Stone>> = aoc::parse_with!(multi::many1(line), bufin);
        Ok(input?
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v.0, v.1, i))
            .collect::<Vec<_>>())
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 5);
    Ok(())
}
