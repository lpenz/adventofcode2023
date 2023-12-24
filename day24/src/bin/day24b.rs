// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day24::*;

use std::ops::Add;
use std::ops::Mul;
use z3;
use z3::ast::Ast;
use z3::{ast, Config, Context, Solver};

fn process(bufin: impl BufRead) -> Result<usize> {
    let stones = parser::parse(bufin)?;
    let ctx = &Context::new(&Config::default());
    let solver = Solver::new(ctx);
    let px = &z3::ast::Int::new_const(ctx, "px");
    let py = &z3::ast::Int::new_const(ctx, "py");
    let pz = &z3::ast::Int::new_const(ctx, "pz");
    let vx = &z3::ast::Int::new_const(ctx, "vx");
    let vy = &z3::ast::Int::new_const(ctx, "vy");
    let vz = &z3::ast::Int::new_const(ctx, "vz");
    for (i, stone) in stones.into_iter().enumerate() {
        let ti = z3::ast::Int::new_const(ctx, format!("t{}", i));
        solver.assert(
            &ast::Int::from_i64(ctx, stone.0 .0)
                .add(&ast::Int::from_i64(ctx, stone.1 .0).mul(&ti))
                ._eq(&px.add(&vx.mul(&ti))),
        );
        solver.assert(
            &ast::Int::from_i64(ctx, stone.0 .1)
                .add(&ast::Int::from_i64(ctx, stone.1 .1).mul(&ti))
                ._eq(&py.add(&vy.mul(&ti))),
        );
        solver.assert(
            &ast::Int::from_i64(ctx, stone.0 .2)
                .add(&ast::Int::from_i64(ctx, stone.1 .2).mul(&ti))
                ._eq(&pz.add(&vz.mul(&ti))),
        );
    }
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().ok_or_eyre("get_model")?;
    let solution_str = format!(
        "{}",
        model.eval(&px.add(py.add(pz)), true).ok_or_eyre("eval")?
    );
    solution_str.parse().map_err(Report::new)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 47);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
