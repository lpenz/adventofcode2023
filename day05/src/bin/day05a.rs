// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day05::*;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (seeds, almanac) = input;
    seeds
        .into_iter()
        .map(|seed| {
            almanac.iter().fold(seed, |seed, map| {
                map.iter()
                    .find_map(|entry| {
                        let (dest, source, len) = *entry;
                        #[allow(clippy::unnecessary_lazy_evaluations)]
                        (source <= seed && seed <= source + len).then(|| seed - source + dest)
                    })
                    .unwrap_or(seed)
            })
        })
        .min()
        .ok_or_else(|| eyre!("no seed found"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 35);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
