// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

pub use sqrid::Dir;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn direction(input: &str) -> IResult<&str, Dir> {
        let (input, d) = character::one_of("UDLR")(input)?;
        let d = match d {
            'U' => Dir::N,
            'R' => Dir::E,
            'D' => Dir::S,
            'L' => Dir::W,
            _ => unreachable!(),
        };
        Ok((input, d))
    }

    fn meters(input: &str) -> IResult<&str, i32> {
        let (input, meters) = character::i32(input)?;
        Ok((input, meters))
    }

    fn color(input: &str) -> IResult<&str, u32> {
        let (input, _) = bytes::tag("(#")(input)?;
        let (input, c) = character::hex_digit1(input)?;
        let (input, _) = bytes::tag(")")(input)?;
        let c = u32::from_str_radix(c, 16).unwrap();
        Ok((input, c))
    }

    fn line(input: &str) -> IResult<&str, (Dir, i32, u32)> {
        let (input, d) = direction(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, m) = meters(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, c) = color(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (d, m, c)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Dir, i32, u32)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 14);
    Ok(())
}
