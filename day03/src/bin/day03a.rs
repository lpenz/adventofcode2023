// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

use sqrid::postrait::PosT;

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let pos = Pos::try_from((x as u16, y as u16))?;
            grid[pos] = cell;
        }
    }
    let mut numbers = vec![];
    // Look for symbols:
    for qa_symbol in Pos::iter() {
        if !matches!(grid[qa_symbol], Cell::Symbol(_)) {
            continue;
        }
        // Check adjacencies, with diagonals:
        for dir in Dir::iter::<true>() {
            let Ok(qa_adj) = qa_symbol + dir else {
                continue;
            };
            let Ok(number) = grid_get_number(&mut grid, qa_adj) else {
                continue;
            };
            numbers.push(number);
        }
    }
    Ok(numbers.into_iter().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 4361);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
