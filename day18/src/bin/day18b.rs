// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day18::*;

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    let instructions = input
        .into_iter()
        .map(|(_, _, color)| color2instr(color))
        .collect::<Result<Vec<_>>>()?;
    calc_area(instructions)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 952408144115);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
