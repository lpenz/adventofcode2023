// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

pub type Entry = (usize, usize, usize);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn seeds(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, _) = bytes::tag("seeds: ")(input)?;
        let (input, seeds) = multi::separated_list1(bytes::tag(" "), character::u64)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, seeds.into_iter().map(|s| s as usize).collect()))
    }

    fn triplett_line(input: &str) -> IResult<&str, Entry> {
        let (input, num1) = character::u64(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, num2) = character::u64(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, num3) = character::u64(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (num1 as usize, num2 as usize, num3 as usize)))
    }

    fn onemap(input: &str) -> IResult<&str, Vec<Entry>> {
        let (input, _name) = multi::many1(character::none_of(" "))(input)?;
        let (input, _) = bytes::tag(" map:\n")(input)?;
        let (input, entries) = multi::many1(triplett_line)(input)?;
        Ok((input, entries))
    }

    fn parse_all(input: &str) -> IResult<&str, (Vec<usize>, Vec<Vec<Entry>>)> {
        let (input, seeds) = seeds(input)?;
        let (input, _) = character::newline(input)?;
        let (input, maps) = multi::separated_list1(character::newline, onemap)(input)?;
        Ok((input, (seeds, maps)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<usize>, Vec<Vec<Entry>>)> {
        aoc::parse_with!(parse_all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 4);
    assert_eq!(input.1.len(), 7);
    Ok(())
}
