// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

fn process(inc: i64, bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    calc_distances(inc, input)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(9, EXAMPLE.as_bytes())?, 1030);
    assert_eq!(process(99, EXAMPLE.as_bytes())?, 8410);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(999_999, stdin().lock()))
}
