// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::cmp::min;
use std::collections::HashSet;

pub const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range(i64, i64);

impl Range {
    pub fn new(a: i64, b: i64) -> Self {
        Range(min(a, b), std::cmp::max(a, b))
    }
    pub fn size(&self) -> i64 {
        (self.1 - self.0) + 1
    }
    pub fn min(&self) -> i64 {
        self.0
    }
    pub fn max(&self) -> i64 {
        self.1
    }
    pub fn contains(&self, value: i64) -> bool {
        self.0 <= value && value <= self.1
    }
    pub fn intersection(&self, other: &Range) -> Option<Range> {
        if self.0 <= other.0 && other.0 <= self.1 {
            Some(Range::new(other.0, min(other.1, self.1)))
        } else if other.0 <= self.0 && self.0 <= other.1 {
            Some(Range::new(self.0, min(other.1, self.1)))
        } else {
            None
        }
    }
}

pub type Brick = (Range, Range, Range);

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn point(input: &str) -> IResult<&str, (i64, i64, i64)> {
        let (input, x) = character::i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = character::i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, z) = character::i64(input)?;
        Ok((input, (x, y, z)))
    }

    fn line(input: &str) -> IResult<&str, Brick> {
        let (input, p0) = point(input)?;
        let (input, _) = tag("~")(input)?;
        let (input, p1) = point(input)?;
        let (input, _) = character::newline(input)?;
        Ok((
            input,
            (
                Range::new(p0.0, p1.0),
                Range::new(p0.1, p1.1),
                Range::new(p0.2, p1.2),
            ),
        ))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Brick>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 7);
    Ok(())
}

pub fn brick_on(bricks: &[Brick], b: &Brick) -> Option<Brick> {
    if b.2.min() == 1 {
        return None;
    }
    bricks
        .iter()
        .filter(|other| {
            b.0.intersection(&other.0).is_some()
                && b.1.intersection(&other.1).is_some()
                && other.2.max() < b.2.min()
        })
        .max_by_key(|o| o.2.max())
        .copied()
}

pub fn falls_to(bricks: &[Brick], b: &Brick) -> Option<i64> {
    if let Some(other) = brick_on(bricks, b) {
        if b.2.min() != other.2.max() + 1 {
            Some(other.2.max() + 1)
        } else {
            None
        }
    } else {
        (b.2.min() != 1).then_some(1)
    }
}

pub fn settle_bricks(bricks: &mut [Brick]) -> usize {
    let mut changed = true;
    let mut fell = HashSet::<usize>::new();
    while changed {
        let old = bricks.to_vec();
        changed = false;
        for (i, b) in bricks.iter_mut().enumerate() {
            if let Some(z) = falls_to(&old, b) {
                let height = b.2.max() - b.2.min();
                b.2 = Range::new(z, z + height);
                fell.insert(i);
                changed = true;
            }
        }
    }
    fell.len()
}
