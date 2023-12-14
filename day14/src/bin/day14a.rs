// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day14::*;

pub use sqrid::Qr;
pub type Sqrid = sqrid::sqrid_create!(100, 100, false);
pub type Qa = sqrid::qa_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub type SqridDebug = sqrid::sqrid_create!(10, 10, false);
pub type QaDebug = sqrid::qa_create!(SqridDebug);
pub type GridDebug = sqrid::grid_create!(SqridDebug, Cell);

fn process(rows: usize, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
        }
    }
    for qa_rock in Qa::iter() {
        if grid[qa_rock] != Cell::Rock {
            continue;
        }
        let mut qa = qa_rock;
        while let Ok(qa_new) = qa + Qr::N {
            if grid[qa_new] != Cell::Empty {
                break;
            }
            qa = qa_new;
        }
        if qa != qa_rock {
            grid[qa_rock] = Cell::Empty;
            grid[qa] = Cell::Rock;
        }
    }
    let g = QaDebug::iter()
        .map(|qa| grid[Qa::try_from(qa.tuple()).unwrap()])
        .collect::<GridDebug>();
    eprintln!("{}", g);
    Ok(Qa::iter()
        .map(|qa| {
            if grid[qa] == Cell::Rock {
                let t = qa.tuple();
                rows - t.1 as usize
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 136);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(100, stdin().lock())?);
    Ok(())
}
