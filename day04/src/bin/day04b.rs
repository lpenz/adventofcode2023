// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::BTreeMap;
use std::io::{stdin, BufRead};

use day04::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let num_cards = input.len();
    let matches = input
        .into_iter()
        .enumerate()
        .map(|(id, (winners, have))| (id, have.into_iter().filter(|h| winners.contains(h)).count()))
        .collect::<BTreeMap<_, _>>();
    let mut copies = BTreeMap::<usize, usize>::default();
    for (id0, matches) in matches {
        let c = copies.entry(id0).or_insert(0);
        // Add one for the original card:
        *c += 1;
        let card_copies = *c;
        for id in (id0 + 1)..std::cmp::min(num_cards, id0 + matches + 1) {
            let e = copies.entry(id).or_insert(0);
            *e += card_copies;
        }
    }
    Ok(copies.into_values().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 30);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
