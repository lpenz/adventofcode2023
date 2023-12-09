// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day08::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let (instrs, paths) = parser::parse(bufin)?;
    let paths = paths.into_iter().collect::<HashMap<_, _>>();
    let starts = paths
        .keys()
        .filter(|n| n.last_letter() == 'A')
        .copied()
        .collect::<Vec<_>>();
    let cycle_lengths = starts
        .into_par_iter()
        .map(|initial| {
            let mut curr = initial;
            instrs
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(steps, instr)| {
                    curr = Node::follow(&paths[&curr], instr);
                    (curr.last_letter() == 'Z').then_some(steps + 1)
                })
                .unwrap_or_else(|| unreachable!())
        })
        .collect::<Vec<_>>();
    Ok(cycle_lengths.into_iter().reduce(num::integer::lcm).unwrap())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 2);
    assert_eq!(process(EXAMPLE2.as_bytes())?, 6);
    assert_eq!(process(EXAMPLE3.as_bytes())?, 6);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
