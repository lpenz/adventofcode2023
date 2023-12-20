// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

// use std::collections::BTreeMap;

pub const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

pub type Wname = copstr::Str<3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Par {
    X,
    M,
    A,
    S,
}

impl Par {
    pub const ALL: [Par; 4] = [Par::X, Par::M, Par::A, Par::S];
}

impl TryFrom<char> for Par {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Par::X),
            'm' => Ok(Par::M),
            'a' => Ok(Par::A),
            's' => Ok(Par::S),
            other => Err(eyre!("invalid par {}", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Goto(Wname),
    Accept,
    Reject,
}

impl Action {
    pub fn get_wname(&self) -> Option<Wname> {
        match self {
            Action::Goto(wname) => Some(*wname),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Lt,
    Gt,
}

impl TryFrom<char> for Op {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Op::Lt),
            '>' => Ok(Op::Gt),
            other => Err(eyre!("invalid op {}", other)),
        }
    }
}

#[derive(Debug)]
pub struct Cond {
    pub par: Par,
    pub op: Op,
    pub value: u64,
}

impl Cond {
    pub fn matchh(&self, part: &Part) -> bool {
        let part_value = part.get(self.par);
        match self.op {
            Op::Lt => part_value < self.value,
            Op::Gt => part_value > self.value,
        }
    }
    pub fn is(&self, par: Par, op: Op) -> bool {
        self.par == par && self.op == op
    }
}

#[derive(Debug)]
pub struct Rule {
    pub cond: Option<Cond>,
    pub act: Action,
}

impl Rule {
    pub fn matchh(&self, part: &Part) -> bool {
        self.cond.is_none() || self.cond.as_ref().map(|c| c.matchh(part)).unwrap()
    }
}

pub struct Workflow {
    pub wname: Wname,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn eval(&self, part: &Part) -> Option<Action> {
        self.rules
            .iter()
            .find_map(|rule| rule.matchh(part).then_some(rule.act))
    }
}

#[derive(Debug)]
pub struct Part(pub [u64; 4]);

impl Part {
    pub fn get(&self, par: Par) -> u64 {
        self.0[par as usize]
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn wname(input: &str) -> IResult<&str, Wname> {
        let (input, wname_chars) =
            multi::many1(character::satisfy(|c| c.is_ascii_lowercase()))(input)?;
        let wname_str: String = wname_chars.into_iter().collect();
        Ok((input, Wname::try_from(wname_str.as_str()).unwrap()))
    }

    fn par_name(input: &str) -> IResult<&str, Par> {
        let (input, par_char) = character::one_of("xmas")(input)?;
        Ok((input, Par::try_from(par_char).unwrap()))
    }

    fn op(input: &str) -> IResult<&str, Op> {
        let (input, op_char) = character::one_of("<>")(input)?;
        Ok((input, Op::try_from(op_char).unwrap()))
    }

    fn action_ar(input: &str) -> IResult<&str, Action> {
        let (input, act_char) = character::one_of("RA")(input)?;
        Ok((
            input,
            match act_char {
                'A' => Action::Accept,
                'R' => Action::Reject,
                _ => unreachable!(),
            },
        ))
    }

    fn action_goto(input: &str) -> IResult<&str, Action> {
        let (input, name) = lowercase_str(input)?;
        Ok((input, Action::Goto(name.as_str().try_into().unwrap())))
    }

    fn action(input: &str) -> IResult<&str, Action> {
        branch::alt((action_ar, action_goto))(input)
    }

    fn rule_cond(input: &str) -> IResult<&str, Rule> {
        let (input, pname) = par_name(input)?;
        let (input, op_) = op(input)?;
        let (input, value) = character::u64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, act) = action(input)?;
        Ok((
            input,
            Rule {
                cond: Some(Cond {
                    par: pname,
                    op: op_,
                    value,
                }),
                act,
            },
        ))
    }

    fn rule_nocond(input: &str) -> IResult<&str, Rule> {
        let (input, act) = action(input)?;
        Ok((input, Rule { cond: None, act }))
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        branch::alt((rule_cond, rule_nocond))(input)
    }

    fn workflow(input: &str) -> IResult<&str, Workflow> {
        let (input, w_wname) = wname(input)?;
        let (input, _) = bytes::tag("{")(input)?;
        let (input, rules) = multi::separated_list1(tag(","), rule)(input)?;
        let (input, _) = bytes::tag("}\n")(input)?;
        Ok((
            input,
            Workflow {
                wname: w_wname,
                rules,
            },
        ))
    }

    fn part(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("{x=")(input)?;
        let (input, x) = character::u64(input)?;
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = character::u64(input)?;
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = character::u64(input)?;
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = character::u64(input)?;
        let (input, _) = tag("}\n")(input)?;
        Ok((input, Part([x, m, a, s])))
    }

    fn parse_all(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
        let (input, workflows) = multi::many1(workflow)(input)?;
        let (input, _) = character::newline(input)?;
        let (input, parts) = multi::many1(part)(input)?;
        Ok((input, (workflows, parts)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<(Vec<Workflow>, Vec<Part>)> {
        aoc::parse_with!(parse_all, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.0.len(), 11);
    assert_eq!(input.1.len(), 5);
    Ok(())
}
