// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day21::*;

use std::collections::HashSet;

pub use sqrid::Dir;
pub type Steps = i64;

pub type P = (i64, i64);

fn go(p: P, dir: Dir) -> P {
    (
        p.0 + match dir {
            Dir::W => -1,
            Dir::E => 1,
            _ => 0,
        },
        p.1 + match dir {
            Dir::N => -1,
            Dir::S => 1,
            _ => 0,
        },
    )
}

fn into_map(size: i64, input: Vec<Vec<Cell>>) -> (P, HashSet<P>) {
    let mut rocks = <HashSet<_>>::new();
    let mut start = (0_i64, 0_i64);
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let t = (x as i64, y as i64);
            if cell == Cell::Start {
                start = t;
            } else if cell == Cell::Rock {
                rocks.insert(t);
                rocks.insert((-size + t.0, t.1));
                rocks.insert((t.0, -size + t.1));
                rocks.insert((-size + t.0, -size + t.1));
            }
        }
    }
    (start, rocks)
}

fn process(size: i64, steps: Steps, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (start, rocks) = into_map(size, input);
    let mut frontier = [start].into_iter().collect::<HashSet<(i64, i64)>>();
    let wanted = [65, 196, 327];
    let mut coefs = vec![];
    for i in 0..steps {
        if wanted.contains(&i) {
            eprintln!("{} {}", i, frontier.len());
            coefs.push(frontier.len())
        }
        if i == wanted[wanted.len() - 1] {
            break;
        }
        frontier = frontier
            .into_iter()
            .flat_map(|p| {
                let rocks = &rocks;
                Dir::ALL4.into_iter().filter_map(move |d| {
                    let newp = go(p, d);
                    let basep = (newp.0 % size, newp.1 % size);
                    (!rocks.contains(&basep)).then_some(newp)
                })
            })
            .collect();
    }
    Ok(frontier.len())
}

fn main() -> Result<()> {
    do_main(|| process(131, 26501365, stdin().lock()))
}
