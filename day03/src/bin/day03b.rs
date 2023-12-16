// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
        }
    }
    let mut numbers = vec![];
    // Look for symbols:
    for qa_symbol in Qa::iter() {
        if grid[qa_symbol] != Cell::Symbol('*') {
            continue;
        }
        // Check adjacencies, with diagonals:
        let mut this_gear = vec![];
        for qr in Qr::iter::<true>() {
            let Ok(qa_adj) = qa_symbol + qr else { continue };
            let Ok(number) = grid_get_number(&mut grid, qa_adj) else {
                continue;
            };
            this_gear.push(number);
        }
        if this_gear.len() != 2 {
            // Not a gear
            continue;
        }
        numbers.push(this_gear[0] * this_gear[1]);
    }
    Ok(numbers.into_iter().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 467835);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
