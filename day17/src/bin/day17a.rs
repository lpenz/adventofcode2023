// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day17::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

pub type Heat = u32;

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct State {
    pub pos: Pos,
    pub lastdir: Option<Dir>,
    pub dircount: usize,
}

fn process(size: u16, bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let gheat = Grid::try_from(input)?;
    let mut frontier = BinaryHeap::<(Reverse<Heat>, State)>::default();
    frontier.push((Reverse(0), State::default()));
    let mut visited = HashSet::<State>::default();
    let mut heatacummap = HashMap::<State, Heat>::default();
    heatacummap.insert(State::default(), 0);
    let goal = Pos::try_from((size - 1, size - 1)).unwrap();
    while let Some((_priority, st)) = frontier.pop() {
        let pos = st.pos;
        if visited.contains(&st) {
            continue;
        }
        let heatacum = heatacummap[&st];
        if pos == goal {
            return Ok(heatacum);
        }
        for dir in Dir::iter::<false>() {
            if st.lastdir == Some(-dir) || (st.lastdir == Some(dir) && st.dircount >= 3) {
                // Gets wobbly
                continue;
            }
            let Ok(newpos) = pos + dir else { continue };
            let t = newpos.tuple();
            if t.0 >= size || t.1 >= size {
                continue;
            }
            let heatacum = heatacum + gheat[newpos];
            let dircount = if Some(dir) == st.lastdir {
                st.dircount + 1
            } else {
                1
            };
            let newst = State {
                pos: newpos,
                lastdir: Some(dir),
                dircount,
            };
            let e = heatacummap.entry(newst).or_insert(heatacum);
            if heatacum < *e {
                *e = heatacum;
            }
            if visited.contains(&newst) {
                continue;
            }
            let dist = Pos::manhattan(&newpos, &goal) as u32;
            let priority = Reverse(heatacum + dist);
            frontier.push((priority, newst));
        }
        visited.insert(st);
    }
    unreachable!();
}

#[test]
fn test() -> Result<()> {
    let start = std::time::Instant::now();
    assert_eq!(process(13, EXAMPLE.as_bytes())?, 102);
    println!("Elapsed: {}", elapsed(&start));
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(141, stdin().lock()))
}
