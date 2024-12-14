// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day10::*;

use sqrid::postrait::PosT;

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy)]
pub struct P(pub f64, pub f64);

impl From<Pos> for P {
    fn from(pos: Pos) -> P {
        (&pos).into()
    }
}

impl From<&Pos> for P {
    fn from(pos: &Pos) -> P {
        let t = pos.tuple();
        P(t.0 as f64, t.1 as f64)
    }
}

pub fn ccw(a: P, b: P, c: P) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LineSeg(pub P, pub P);

pub fn intersect(u: &LineSeg, v: &LineSeg) -> bool {
    ccw(u.0, v.0, v.1) != ccw(u.1, v.0, v.1) && ccw(u.0, u.1, v.0) != ccw(u.0, u.1, v.1)
}

fn calc_pipe(grid: &Grid, start: Pos) -> Vec<Pos> {
    for qr0 in [Dir::N, Dir::E, Dir::S, Dir::W] {
        let mut pipe = vec![start];
        let mut dir = qr0;
        let mut pos = start;
        while let Ok(next_qa) = pos + dir {
            pos = next_qa;
            pipe.push(pos);
            if pos == start {
                return pipe;
            }
            if let Some(next_qr) = next_qr(grid, pos, dir) {
                dir = next_qr;
            } else {
                break;
            }
        }
    }
    unreachable!()
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    let mut start = Pos::default();
    let botright = Pos::try_from((input[0].len() as u16 - 1, input.len() as u16 - 1)).unwrap();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let pos = Pos::try_from((x as u16, y as u16))?;
            grid[pos] = cell;
            if cell == Cell::Start {
                start = pos;
            }
        }
    }
    let pipe = calc_pipe(&grid, start);
    let linesegs = pipe
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| LineSeg(a.into(), b.into()))
        .chain(std::iter::once(LineSeg(
            pipe[pipe.len() - 1].into(),
            pipe[0].into(),
        )))
        .collect::<Vec<_>>();
    Ok(Pos::iter_range(Pos::TOP_LEFT, botright)
        .filter(|pos| !pipe.contains(pos))
        .filter(|pos| {
            let qaline = LineSeg(pos.into(), P::default());
            let intersections = linesegs
                .iter()
                .filter(|polyline| intersect(&qaline, polyline))
                .count();
            intersections % 2 == 1
        })
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE3.as_bytes())?, 4);
    assert_eq!(process(EXAMPLE4.as_bytes())?, 10);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
