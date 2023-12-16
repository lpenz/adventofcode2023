// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day08::*;

use std::collections::HashMap;
use std::str::FromStr;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (instrs, paths) = parser::parse(bufin)?;
    let paths = paths.into_iter().collect::<HashMap<_, _>>();
    let mut curr = Node::from_str("AAA")?;
    let target = Node::from_str("ZZZ")?;
    instrs
        .into_iter()
        .cycle()
        .enumerate()
        .find_map(|(steps, instr)| {
            curr = Node::follow(&paths[&curr], &instr);
            (curr == target).then_some(steps + 1)
        })
        .ok_or_else(|| unreachable!())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 2);
    assert_eq!(process(EXAMPLE2.as_bytes())?, 6);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
