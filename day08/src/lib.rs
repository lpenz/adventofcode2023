// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::{eyre::eyre, Report, Result};
use std::str::FromStr;

pub const EXAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

pub const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

pub const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Node(pub [char; 3]);

impl Node {
    pub fn follow(opts: &(Node, Node), instr: &Instr) -> Node {
        match instr {
            Instr::L => opts.0,
            Instr::R => opts.1,
        }
    }

    pub fn last_letter(&self) -> char {
        self.0[2]
    }
}

impl FromStr for Node {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Node::try_from(s.chars().collect::<Vec<_>>().as_ref())
    }
}

impl TryFrom<&[char]> for Node {
    type Error = Report;
    fn try_from(chars: &[char]) -> Result<Self, Self::Error> {
        if chars.len() != 3 {
            return Err(eyre!("invalid length for hand"));
        }
        Ok(Node([chars[0], chars[1], chars[2]]))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instr {
    L,
    R,
}

impl TryFrom<char> for Instr {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Instr::L),
            'R' => Ok(Instr::R),
            other => Err(eyre!("invalid instruction letter {}", other)),
        }
    }
}

pub type Entry = (Node, (Node, Node));

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn instr(input: &str) -> IResult<&str, Instr> {
        let (input, letter) = character::one_of("LR")(input)?;
        Ok((input, Instr::try_from(letter).unwrap()))
    }

    fn node(input: &str) -> IResult<&str, Node> {
        let (input, name) = multi::count(character::satisfy(|c| c.is_ascii()), 3)(input)?;
        Ok((input, Node::try_from(name.as_ref()).unwrap()))
    }

    fn connection(input: &str) -> IResult<&str, Entry> {
        let (input, orig) = node(input)?;
        let (input, _) = bytes::tag(" = (")(input)?;
        let (input, dir1) = node(input)?;
        let (input, _) = bytes::tag(", ")(input)?;
        let (input, dir2) = node(input)?;
        let (input, _) = bytes::tag(")\n")(input)?;
        Ok((input, (orig, (dir1, dir2))))
    }

    fn parse_all(input: &str) -> IResult<&str, (Vec<Instr>, Vec<Entry>)> {
        let (input, instrs) = multi::many1(instr)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, _) = character::newline(input)?;
        let (input, connections) = multi::many1(connection)(input)?;
        Ok((input, (instrs, connections)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Instr>, Vec<Entry>)> {
        aoc::parse_with!(parse_all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE1.as_bytes())?.0.len(), 2);
    assert_eq!(parser::parse(EXAMPLE1.as_bytes())?.1.len(), 7);
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?.0.len(), 3);
    assert_eq!(parser::parse(EXAMPLE2.as_bytes())?.1.len(), 3);
    assert_eq!(parser::parse(EXAMPLE3.as_bytes())?.0.len(), 2);
    assert_eq!(parser::parse(EXAMPLE3.as_bytes())?.1.len(), 8);
    Ok(())
}
