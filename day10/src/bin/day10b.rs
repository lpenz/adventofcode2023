// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day10::*;

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy)]
pub struct P(pub f64, pub f64);

impl From<Qa> for P {
    fn from(qa: Qa) -> P {
        (&qa).into()
    }
}

impl From<&Qa> for P {
    fn from(qa: &Qa) -> P {
        let t = qa.tuple();
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

fn calc_pipe(grid: &Grid, start: Qa) -> Vec<Qa> {
    for qr0 in [Qr::N, Qr::E, Qr::S, Qr::W] {
        let mut pipe = vec![start];
        let mut qr = qr0;
        let mut qa = start;
        while let Ok(next_qa) = qa + qr {
            qa = next_qa;
            pipe.push(qa);
            if qa == start {
                return pipe;
            }
            if let Some(next_qr) = next_qr(grid, qa, qr) {
                qr = next_qr;
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
    let mut start = Qa::default();
    let botright = Qa::try_from((input[0].len() as u16 - 1, input.len() as u16 - 1)).unwrap();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
            if cell == Cell::Start {
                start = qa;
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
    Ok(Qa::iter_range(Qa::TOP_LEFT, botright)
        .filter(|qa| !pipe.contains(qa))
        .filter(|qa| {
            let qaline = LineSeg(qa.into(), P::default());
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
