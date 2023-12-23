// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day14::*;

fn process(size: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::try_from(input)?;
    grid = tilt(size, grid, Dir::N);
    Ok(grid_load(size, &grid))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 136);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(100, stdin().lock()))
}
