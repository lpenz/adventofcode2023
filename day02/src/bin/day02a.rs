// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let limits = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
        .into_iter()
        .collect::<Set>();
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .enumerate()
        .filter_map(|(gameid, game)| {
            game.into_iter()
                .all(|sets| sets.into_iter().all(|(color, num)| num <= limits[&color]))
                .then_some(gameid + 1)
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 8);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
