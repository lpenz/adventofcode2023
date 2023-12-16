// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day06::*;

use std::fmt::Write;

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let (time_vec, distance_vec) = input;
    let time = time_vec
        .into_iter()
        .fold(String::default(), |mut s, t| {
            write!(s, "{}", t).unwrap();
            s
        })
        .parse::<u64>()?;
    let distance = distance_vec
        .into_iter()
        .fold(String::default(), |mut s, t| {
            write!(s, "{}", t).unwrap();
            s
        })
        .parse::<u64>()?;
    Ok((0..=time)
        .map(|button_time| button_time * (time - button_time))
        .filter(|i| i > &distance)
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 71503);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
