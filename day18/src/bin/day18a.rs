// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day18::*;

use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy)]
pub struct P(pub f64, pub f64);

pub fn ccw(a: P, b: P, c: P) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LineSeg(pub P, pub P);

pub fn intersect(u: &LineSeg, v: &LineSeg) -> bool {
    ccw(u.0, v.0, v.1) != ccw(u.1, v.0, v.1) && ccw(u.0, u.1, v.0) != ccw(u.0, u.1, v.1)
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut point: (i32, i32) = (0, 0);
    let mut points = vec![point];
    let mut dug = points.iter().copied().collect::<HashSet<_>>();
    let mut linesegs = vec![];
    let mut xlimits = autofolder::MinMax::new(point.0);
    let mut ylimits = autofolder::MinMax::new(point.1);
    for (dir, meters, _) in &input {
        let old = point;
        for _ in 0..*meters {
            match dir {
                Dir::N => {
                    point.1 -= 1;
                }
                Dir::E => {
                    point.0 += 1;
                }
                Dir::S => {
                    point.1 += 1;
                }
                Dir::W => {
                    point.0 -= 1;
                }
                _ => unreachable!(),
            }
            dug.insert(point);
        }
        points.push(point);
        linesegs.push(LineSeg(
            P(old.0 as f64, old.1 as f64),
            P(point.0 as f64, point.1 as f64),
        ));
        xlimits.reduce(point.0);
        ylimits.reduce(point.1);
    }
    eprintln!("xlimits {:?}", xlimits);
    eprintln!("ylimits {:?}", ylimits);
    let xlimits = xlimits.to_inner().unwrap();
    let ylimits = ylimits.to_inner().unwrap();
    let reference = P(xlimits.1 as f64 * 50_f64, ylimits.1 as f64 * 50_f64);
    for y in ylimits.0..=ylimits.1 {
        for x in xlimits.0..=xlimits.1 {
            let point = (x, y);
            if dug.contains(&point) {
                continue;
            }
            let p = P(x as f64, y as f64);
            let line = LineSeg(p, reference);
            let intersections = linesegs
                .iter()
                .filter(|polyline| intersect(&line, polyline))
                .count();
            if intersections % 2 == 1 {
                dug.insert(point);
            }
        }
    }
    // eprintln!("{:?}", dug);
    Ok(dug.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 62);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
