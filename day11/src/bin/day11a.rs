// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

use rayon::prelude::*;
use std::collections::HashSet;

pub use sqrid::Qr;
pub type Sqrid = sqrid::sqrid_create!(140, 140, false);
pub type Qa = sqrid::qa_create!(Sqrid);

fn traverse(
    expand_x: &HashSet<u16>,
    expand_y: &HashSet<u16>,
    pos: Qa,
    dir: sqrid::Qr,
) -> Option<(Qa, usize)> {
    let t = pos.tuple();
    let mut cost = 1;
    if (dir == Qr::W || dir == Qr::E) && expand_x.contains(&t.0) {
        cost = 2;
    }
    if (dir == Qr::N || dir == Qr::S) && expand_y.contains(&t.1) {
        cost = 2;
    }
    let next_pos = (pos + dir).ok()?;
    Some((next_pos, cost))
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let expand_x = (0..input[0].len())
        .filter(|x| input.iter().all(|line| line[*x] == Cell::Empty))
        .map(|v| v as u16)
        .collect::<HashSet<_>>();
    let expand_y = (0..input.len())
        .filter(|y| input[*y].iter().all(|cell| cell == &Cell::Empty))
        .map(|v| v as u16)
        .collect::<HashSet<_>>();
    let galaxies = Qa::iter()
        .filter(|qa| {
            let t = qa.tuple();
            let t = (t.0 as usize, t.1 as usize);
            t.1 < input.len() && t.0 < input[0].len() && input[t.1][t.0] == Cell::Galaxy
        })
        .collect::<Vec<_>>();
    let gpairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| galaxies[i + 1..].iter().map(move |g2| (g1, g2)))
        .collect::<Vec<_>>();
    Ok(gpairs
        .par_iter()
        .map(|(g1, g2)| {
            if let Ok(path) =
                Sqrid::ucs_path_hash(|pos, dir| traverse(&expand_x, &expand_y, pos, dir), g1, g2)
            {
                let mut cost = 0;
                path.iter().fold(**g1, |pos, dir| {
                    let (next, c) = traverse(&expand_x, &expand_y, pos, *dir).unwrap();
                    cost += c;
                    next
                });
                cost
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 374);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
