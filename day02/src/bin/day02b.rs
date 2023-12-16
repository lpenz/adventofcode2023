// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

use rayon::prelude::*;

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_par_iter()
        .map(|game| {
            game.into_par_iter()
                .reduce(Set::new, |maxset, sets| {
                    sets.into_iter().fold(maxset, |mut maxset, (color, num)| {
                        let e = maxset.entry(color).or_default();
                        *e = std::cmp::max(*e, num);
                        maxset
                    })
                })
                .into_par_iter()
                .map(|(_, num)| num)
                .product::<u32>()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 2286);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
