// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day18::*;

fn process(bufin: impl BufRead) -> Result<i64> {
    let instructions = parser::parse(bufin)?
        .into_iter()
        .map(|(dir, meters, _)| (dir, meters))
        .collect::<Vec<_>>();
    calc_area(instructions)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 62);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
