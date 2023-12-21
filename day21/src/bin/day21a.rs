// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day21::*;

use std::collections::HashSet;

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(131, 131, false);
// pub type Sqrid = sqrid::sqrid_create!(11, 11, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

pub type GridDebug = sqrid::grid_create!(Sqrid, char);

pub type Steps = i32;

fn bfs(
    size: u16,
    grid: &Grid,
    pos: Pos,
    visited: &mut HashSet<(Pos, Steps)>,
    end: &mut HashSet<Pos>,
    stepsleft: i32,
) {
    if visited.contains(&(pos, stepsleft)) {
        return;
    }
    if stepsleft == 0 {
        end.insert(pos);
        return;
    }
    for dir in Dir::iter::<false>() {
        let Ok(newpos) = pos + dir else { continue };
        let newpos_t = newpos.tuple();
        if newpos_t.0 >= size || newpos_t.1 >= size {
            continue;
        }
        if grid[newpos] == Cell::Rock {
            continue;
        }
        bfs(size, grid, newpos, visited, end, stepsleft - 1);
    }
    visited.insert((pos, stepsleft));
}

fn process(size: u16, steps: Steps, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input)?;
    let start = Pos::iter().find(|p| grid[p] == Cell::Start).unwrap();
    let mut visited = HashSet::<(Pos, Steps)>::new();
    let mut end = HashSet::<Pos>::new();
    bfs(size, &grid, start, &mut visited, &mut end, steps);
    Ok(end.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(11, 6, EXAMPLE.as_bytes())?, 16);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(131, 64, stdin().lock()))
}
