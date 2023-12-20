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

    fn meters(input: &str) -> IResult<&str, i64> {
        let (input, meters) = character::i64(input)?;
        Ok((input, meters))
    }

    fn color(input: &str) -> IResult<&str, u32> {
        let (input, _) = bytes::tag("(#")(input)?;
        let (input, c) = character::hex_digit1(input)?;
        let (input, _) = bytes::tag(")")(input)?;
        let c = u32::from_str_radix(c, 16).unwrap();
        Ok((input, c))
    }

    fn line(input: &str) -> IResult<&str, (Dir, i64, u32)> {
        let (input, d) = direction(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, m) = meters(input)?;
        let (input, _) = bytes::tag(" ")(input)?;
        let (input, c) = color(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (d, m, c)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Dir, i64, u32)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 14);
    Ok(())
}

pub type Point = (i64, i64);

pub fn color2instr(color: u32) -> Result<(Dir, i64)> {
    let colorstr = format!("{:x}", color);
    let mut chars = colorstr.chars();
    let dirchar = chars.next_back().ok_or_eyre("chars.next_back")?;
    let dir = match dirchar {
        '0' => Dir::E,
        '1' => Dir::S,
        '2' => Dir::W,
        '3' => Dir::N,
        _ => {
            return Err(eyre!("could not parse dir char {}", dirchar));
        }
    };
    let diststr = chars.as_str();
    Ok((dir, i64::from_str_radix(diststr, 16)?))
}

pub fn instructions2points(instructions: &[(Dir, i64)]) -> Vec<(i64, i64)> {
    let mut points = vec![];
    instructions
        .iter()
        .fold((0_i64, 0_i64), |last, &(dir, meters)| {
            let next = (
                last.0
                    + match dir {
                        Dir::W => -meters,
                        Dir::E => meters,
                        _ => 0,
                    },
                last.1
                    + match dir {
                        Dir::N => -meters,
                        Dir::S => meters,
                        _ => 0,
                    },
            );
            points.push(next);
            next
        });
    points
}

pub fn calc_area(instructions: Vec<(Dir, i64)>) -> Result<i64> {
    let perimeter = instructions
        .iter()
        .map(|(_, meters)| meters)
        .copied()
        .sum::<i64>();
    let points = instructions
        .into_iter()
        .scan((0_i64, 0_i64), |point, (dir, meters)| {
            let old = *point;
            *point = (
                point.0
                    + match dir {
                        Dir::E => meters,
                        Dir::W => -meters,
                        _ => 0,
                    },
                point.1
                    + match dir {
                        Dir::S => meters,
                        Dir::N => -meters,
                        _ => 0,
                    },
            );
            Some(old)
        })
        .collect::<Vec<_>>();
    // Shoelace formula:
    let mut area = 0_f64;
    for i in 0..points.len() {
        let i0 = i;
        let i1 = (i + 1) % (points.len());
        let x0 = points[i0].0 as f64;
        let x1 = points[i1].0 as f64;
        let y0 = points[i0].1 as f64;
        let y1 = points[i1].1 as f64;
        area += (x0 * y1 - x1 * y0) / 2_f64;
    }
    // Pick's theorem:
    Ok((area + perimeter as f64 / 2_f64 + 1_f64).round() as i64)
}

#[test]
fn test4() -> Result<()> {
    let instructions = vec![(Dir::E, 1), (Dir::S, 1), (Dir::W, 1), (Dir::N, 1)];
    assert_eq!(calc_area(instructions)?, 4);
    Ok(())
}

#[test]
fn test9() -> Result<()> {
    let instructions = vec![(Dir::E, 2), (Dir::S, 2), (Dir::W, 2), (Dir::N, 2)];
    assert_eq!(calc_area(instructions)?, 9);
    Ok(())
}

#[test]
fn test_c() -> Result<()> {
    let instructions = vec![
        (Dir::E, 4),
        (Dir::S, 2),
        (Dir::W, 2),
        (Dir::S, 2),
        (Dir::E, 2),
        (Dir::S, 2),
        (Dir::W, 4),
        (Dir::N, 6),
    ];
    assert_eq!(calc_area(instructions)?, 7 * 5 - 2);
    Ok(())
}

#[test]
fn test_c2() -> Result<()> {
    let instructions = vec![
        (Dir::E, 4),
        (Dir::S, 2),
        (Dir::W, 2),
        (Dir::S, 12),
        (Dir::E, 2),
        (Dir::S, 2),
        (Dir::W, 4),
        (Dir::N, 16),
    ];
    assert_eq!(calc_area(instructions)?, 7 * 5 - 2 + 10 * 3);
    Ok(())
}

#[test]
fn test_invc() -> Result<()> {
    let instructions = vec![
        (Dir::N, 6),
        (Dir::E, 4),
        (Dir::S, 2),
        (Dir::W, 2),
        (Dir::S, 2),
        (Dir::E, 2),
        (Dir::S, 2),
        (Dir::W, 4),
    ];
    assert_eq!(calc_area(instructions)?, 7 * 5 - 2);
    Ok(())
}

#[test]
fn test_botleft_missing() -> Result<()> {
    let instructions = vec![
        (Dir::E, 7),
        (Dir::S, 7),
        (Dir::W, 3),
        (Dir::N, 4),
        (Dir::W, 4),
        (Dir::N, 3),
    ];
    assert_eq!(calc_area(instructions)?, 48);
    Ok(())
}
