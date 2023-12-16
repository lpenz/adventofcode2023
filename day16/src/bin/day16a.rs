// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day16::*;

use std::collections::HashSet;

fn process(size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input.clone())?;
    let mut beams = Beams([(Pos::TOP_LEFT, Dir::E)].into_iter().collect());
    beams.process(&grid);
    let mut energized = [Pos::TOP_LEFT].into_iter().collect::<HashSet<Pos>>();
    let mut unchanged = 0;
    while !beams.0.is_empty() && unchanged < size {
        let old = energized.clone();
        beams.next(size, &grid);
        energized.extend(beams.0.iter().map(|(pos, _)| pos));
        if energized != old {
            unchanged = 0;
        } else {
            unchanged += 1;
        }
    }
    Ok(energized.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 46);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(110, stdin().lock()))
}
