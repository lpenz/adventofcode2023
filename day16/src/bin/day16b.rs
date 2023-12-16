// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day16::*;

fn process(size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input.clone())?;
    let mut cache = Cache::default();
    (0..size)
        .flat_map(|i| {
            vec![
                (Pos::new(i, 0).unwrap(), Dir::S),
                (Pos::new(i, size - 1).unwrap(), Dir::N),
                (Pos::new(0, i).unwrap(), Dir::E),
                (Pos::new(size - 1, i).unwrap(), Dir::W),
            ]
            .into_iter()
        })
        .map(|start| calc_energized(size, &grid, &mut cache, start))
        .max()
        .ok_or(eyre!("max not found"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE.as_bytes())?, 51);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(110, stdin().lock()))
}
