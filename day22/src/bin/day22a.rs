// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day22::*;

use rayon::prelude::*;

fn can_disintegrate(bricks: &[Brick], b: &Brick) -> bool {
    let bricks = bricks
        .iter()
        .filter(|o| o != &b)
        .copied()
        .collect::<Vec<_>>();
    bricks.par_iter().all(|o| falls_to(&bricks, o).is_none())
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let mut bricks = parser::parse(bufin)?;
    settle_bricks(&mut bricks);
    Ok(bricks
        .par_iter()
        .filter(|b| can_disintegrate(&bricks, b))
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 5);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
