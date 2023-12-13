// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};

pub const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

pub type VecGrid = Vec<Vec<bool>>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, bool> {
        let (input, c) = character::one_of(".#")(input)?;
        Ok((input, c == '#'))
    }

    fn line(input: &str) -> IResult<&str, Vec<bool>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    fn grid(input: &str) -> IResult<&str, VecGrid> {
        let (input, g) = multi::many1(line)(input)?;
        Ok((input, g))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<VecGrid>> {
        aoc::parse_with!(multi::separated_list1(character::newline, grid), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 2);
    assert_eq!(input[0].len(), 7);
    assert_eq!(input[0][1].len(), 9);
    assert_eq!(input[1].len(), 7);
    assert_eq!(input[1][1].len(), 9);
    Ok(())
}
