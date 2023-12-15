// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Result};
pub use std::fmt;

pub const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Label(pub String);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum Op {
    Del,
    Focus(u32),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Del => write!(f, "-"),
            Op::Focus(fl) => write!(f, "={}", fl),
        }
    }
}

pub struct Step {
    pub label: Label,
    pub op: Op,
}

impl Step {
    pub fn new(label: Label, op: Op) -> Self {
        Step { label, op }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.label.0, self.op)
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn label(input: &str) -> IResult<&str, Label> {
        let (input, chars) = multi::many1(character::none_of(",-=\n"))(input)?;
        Ok((input, Label(chars.into_iter().collect::<String>())))
    }

    fn op(input: &str) -> IResult<&str, Op> {
        let (input, op) = character::one_of("-=")(input)?;
        if op == '-' {
            Ok((input, Op::Del))
        } else {
            let (input, fl) = character::u32(input)?;
            Ok((input, Op::Focus(fl)))
        }
    }

    fn step(input: &str) -> IResult<&str, Step> {
        let (input, label) = label(input)?;
        let (input, op) = op(input)?;
        Ok((input, Step::new(label, op)))
    }

    fn all_input(input: &str) -> IResult<&str, Vec<Step>> {
        let (input, steps) = multi::separated_list1(bytes::tag(","), step)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, steps))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Step>> {
        aoc::parse_with!(all_input, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 11);
    Ok(())
}

pub fn hash_str(s: &str) -> usize {
    s.chars().fold(0, |value, c| {
        let ascii = c as usize;
        ((value + ascii) * 17) % 256
    })
}
