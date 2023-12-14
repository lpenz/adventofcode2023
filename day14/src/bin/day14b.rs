// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day14::*;

pub use sqrid::Qr;
pub type Sqrid = sqrid::sqrid_create!(100, 100, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub type SqridD = sqrid::sqrid_create!(10, 10, false);
pub type QaD = sqrid::qa_create!(SqridD);
pub type GridD = sqrid::grid_create!(SqridD, Cell);

const CYCLES: u64 = 1000000000;

fn _grid_display(grid: &Grid) {
    let grid_d = QaD::iter()
        .map(|qa| grid[Qa::try_from(qa.tuple()).unwrap()])
        .collect::<GridD>();
    eprintln!("{}", grid_d);
}

fn process(size: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::try_from(input)?;
    let mut cache = HashMap::<Grid, u64>::default();
    let mut icycle = 0;
    while icycle < CYCLES {
        eprintln!("cycle {}", icycle);
        for qr in [Qr::N, Qr::W, Qr::S, Qr::E] {
            grid = tilt(size, grid, qr);
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
    color_eyre::install()?;
    println!("{}", process(100, stdin().lock())?);
    Ok(())
}
