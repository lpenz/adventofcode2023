// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day07::*;

use itertools::Itertools;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|(hand, bid)| (hand.value(true), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as i64 + 1) * bid)
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 5905);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
