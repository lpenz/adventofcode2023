// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day15::*;

fn hash(steps: &Step) -> usize {
    hash_str(&format!("{}", steps))
}

#[test]
fn test_hash() {
    assert_eq!(hash_str("HASH"), 52);
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input.into_iter().map(|step| hash(&step)).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 1320);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
