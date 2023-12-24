// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day24::*;

fn intercept(v: Stone, u: Stone) -> Option<Xyz> {
    let (pos_v, vel_v, _) = v;
    let (pos_u, vel_u, _) = u;
    let a = vel_v.1 / vel_v.0;
    let b = vel_u.1 / vel_u.0;
    let c = pos_v.1 - pos_v.0 * a;
    let d = pos_u.1 - pos_u.0 * b;
    if a == b {
        return None;
    }
    let x = (d - c) / (a - b);
    let y = a * x + c;
    let t_v = (x - pos_v.0) / vel_v.0;
    if t_v < 0_f64 {
        return None;
    }
    let t_u = (x - pos_u.0) / vel_u.0;
    if t_u < 0_f64 {
        return None;
    }
    Some((x, y, 0_f64))
}

fn process(tl: Xyz, br: Xyz, bufin: impl BufRead) -> Result<usize> {
    let stones = parser::parse(bufin)?;
    Ok((0..stones.len() - 1)
        .flat_map(|i| {
            let stones = &stones;
            (i + 1..stones.len()).filter(move |&j| {
                let a = stones[i];
                let b = stones[j];
                if let Some(c) = intercept(a, b) {
                    c.0 >= tl.0 && c.0 <= br.0 && c.1 >= tl.1 && c.1 <= br.1
                } else {
                    false
                }
            })
        })
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(
        process(
            (7_f64, 7_f64, 0_f64),
            (27_f64, 27_f64, 0_f64),
            EXAMPLE.as_bytes()
        )?,
        2
    );
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| {
        process(
            (200000000000000_f64, 200000000000000_f64, 0_f64),
            (400000000000000_f64, 400000000000000_f64, 0_f64),
            stdin().lock(),
        )
    })
}
