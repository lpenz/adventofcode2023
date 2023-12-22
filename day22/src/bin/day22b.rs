// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day22::*;

use rayon::prelude::*;

fn would_fall(bricks: &[Brick], b: &Brick) -> usize {
    let mut bricks = bricks
        .iter()
        .filter(|o| o != &b)
        .copied()
        .collect::<Vec<_>>();
    settle_bricks(&mut bricks)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut bricks = parser::parse(bufin)?;
    settle_bricks(&mut bricks);
    Ok(bricks.par_iter().map(|b| would_fall(&bricks, b)).sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 7);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
