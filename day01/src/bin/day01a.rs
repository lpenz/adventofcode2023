// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn firstnum(input: &str) -> parser::IResult<&str, u32> {
    let (input, _) = parser::character::alpha0(input)?;
    let (input, numstr) = parser::character::one_of("0123456789")(input)?;
    let (input, _) = parser::character::alphanumeric0(input)?;
    Ok((input, numstr.to_digit(10).unwrap()))
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let lines = parser::parse(bufin)?;
    Ok(lines
        .into_iter()
        .map(|line| {
            let first = parser::all_consuming(firstnum)(&line)
                .map_err(|e| eyre!("error reading first digit {:?}", e))?
                .1;
            let reversed = line.chars().rev().collect::<String>();
            let last = parser::all_consuming(firstnum)(&reversed)
                .map_err(|e| eyre!("error reading last digit {:?}", e))?
                .1;
            Ok(first * 10 + last)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 142);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
