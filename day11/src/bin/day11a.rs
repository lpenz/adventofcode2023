// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    calc_distances(1, input)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 374);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
