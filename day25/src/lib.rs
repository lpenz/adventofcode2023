// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

pub type Node = copstr::Str<3>;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn name(input: &str) -> IResult<&str, Node> {
        let (input, name_str) = character::alpha1(input)?;
        Ok((input, name_str.try_into().unwrap()))
    }

    fn line(input: &str) -> IResult<&str, (Node, Vec<Node>)> {
        let (input, node_name) = name(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, node_children) = multi::separated_list1(tag(" "), name)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (node_name, node_children)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Node, Vec<Node>)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 13);
    Ok(())
}
