// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::collections::BTreeMap;
use std::str::FromStr;

pub const EXAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

pub const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Mname(pub copstr::Str<6>);

impl From<&str> for Mname {
    fn from(s: &str) -> Self {
        Mname(s.try_into().unwrap())
    }
}

impl FromStr for Mname {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mname(copstr::Str::<6>::try_from(s)?))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub enum Mtype {
    #[default]
    None,
    FlipFlop,
    Conjunct,
    Broadcast,
}

impl TryFrom<char> for Mtype {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '%' => Ok(Mtype::FlipFlop),
            '&' => Ok(Mtype::Conjunct),
            other => Err(eyre!("invalid module type {}", other)),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub struct Module {
    pub mname: Mname,
    pub mtype: Mtype,
    pub dsts: Vec<Mname>,
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn mtype(input: &str) -> IResult<&str, Mtype> {
        let (input, mtype_char) = character::one_of("&%")(input)?;
        Ok((input, Mtype::try_from(mtype_char).unwrap()))
    }

    fn mname(input: &str) -> IResult<&str, Mname> {
        let (input, mname) = character::alpha1(input)?;
        Ok((input, mname.into()))
    }

    fn module_line(input: &str) -> IResult<&str, Module> {
        let (input, mtype) = mtype(input)?;
        let (input, name) = mname(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, dsts) = multi::separated_list1(tag(", "), mname)(input)?;
        let (input, _) = newline(input)?;
        Ok((
            input,
            Module {
                mname: name,
                mtype,
                dsts,
            },
        ))
    }

    fn broadcast_line(input: &str) -> IResult<&str, Module> {
        let (input, _) = tag("broadcaster -> ")(input)?;
        let (input, dsts) = multi::separated_list1(tag(", "), mname)(input)?;
        let (input, _) = newline(input)?;
        Ok((
            input,
            Module {
                mname: Mname("0".try_into().unwrap()),
                mtype: Mtype::Broadcast,
                dsts,
            },
        ))
    }

    fn line(input: &str) -> IResult<&str, Module> {
        branch::alt((module_line, broadcast_line))(input)
    }

    fn parse_helper(mut bufin: impl BufRead) -> Result<Vec<Module>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }

    pub fn parse(bufin: impl BufRead) -> Result<BTreeMap<Mname, Module>> {
        let modules = parse_helper(bufin)?;
        let dsts = modules
            .iter()
            .flat_map(|m| m.dsts.iter())
            .copied()
            .collect::<Vec<Mname>>();
        let mut modules = modules
            .into_iter()
            .map(|m| (m.mname, m))
            .collect::<BTreeMap<_, _>>();
        for dst_mname in dsts {
            modules.entry(dst_mname).or_default();
        }
        Ok(modules)
    }
}

#[test]
fn test1() -> Result<()> {
    let input = parser::parse(EXAMPLE1.as_bytes())?;
    assert_eq!(input.len(), 5);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let input = parser::parse(EXAMPLE2.as_bytes())?;
    assert_eq!(input.len(), 6);
    Ok(())
}
