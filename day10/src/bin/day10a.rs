// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day10::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    let mut start = Qa::default();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
            if cell == Cell::Start {
                start = qa;
            }
        }
    }
    for qr0 in [Qr::N, Qr::E, Qr::S, Qr::W] {
        let mut steps = 0;
        let mut qr = qr0;
        let mut qa = start;
        while let Ok(next_qa) = qa + qr {
            qa = next_qa;
            steps += 1;
            if qa == start {
                return Ok(steps / 2);
            }
            if let Some(next_qr) = next_qr(&grid, qa, qr) {
                qr = next_qr;
            } else {
                break;
            }
        }
    }
    Ok(5)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 4);
    assert_eq!(process(EXAMPLE2.as_bytes())?, 8);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
