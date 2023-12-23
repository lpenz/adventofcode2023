// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day23::*;

use std::collections::HashSet;

pub type Sqrid = sqrid::sqrid_create!(141, 141, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

fn dfs(size: u16, grid: &Grid, pos: Pos, visited: &HashSet<Pos>, end: Pos, steps: usize) -> usize {
    if pos == end {
        return steps;
    }
    let pos_t = pos.tuple();
    if pos_t.0 >= size || pos_t.1 >= size {
        return 0;
    }
    if visited.contains(&pos) {
        return 0;
    }
    if grid[pos] == Cell::Wall {
        return 0;
    }
    let mut newvisited = visited.clone();
    newvisited.insert(pos);
    if let Cell::Slope(dir) = grid[pos] {
        let Ok(newpos) = pos + dir else { return 0 };
        dfs(size, grid, newpos, &newvisited, end, steps + 1)
    } else {
        Dir::iter::<false>()
            .filter_map(|dir| {
                (pos + dir)
                    .ok()
                    .map(|newpos| dfs(size, grid, newpos, &newvisited, end, steps + 1))
            })
            .max()
            .unwrap_or(0)
    }
}

fn process(size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input)?;
    let start = Pos::try_from((1, 0))?;
    assert_eq!(grid[start], Cell::Empty);
    let end = Pos::try_from((size - 2, size - 1))?;
    assert_eq!(grid[end], Cell::Empty);
    let visited = Default::default();
    Ok(dfs(size, &grid, start, &visited, end, 0))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(23, EXAMPLE.as_bytes())?, 94);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(141, stdin().lock()))
}
