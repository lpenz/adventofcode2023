// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day14::*;

use std::collections::HashMap;

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(100, 100, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

const CYCLES: u64 = 1000000000;

fn process(size: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::try_from(input)?;
    let mut cache = HashMap::<Grid, u64>::default();
    let mut icycle = 0;
    while icycle < CYCLES {
        for dir in [Dir::N, Dir::W, Dir::S, Dir::E] {
            grid = tilt(size, grid, dir);
        }
        if icycle < CYCLES / 2 {
            if let Some(first) = cache.get(&grid) {
                let diff = icycle - first;
                icycle += diff * ((CYCLES - first) / diff - 1);
            }
            cache.insert(grid, icycle);
        }
        icycle += 1;
    }
    Ok(grid_load(size, &grid))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 64);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(100, stdin().lock()))
}
