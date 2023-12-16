// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use regex::Regex;

use day01::*;

fn readnum(s: &str) -> Result<u32> {
    s.parse::<u32>().or(match s {
        "zero" => Ok(0),
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Err(eyre!("invalid number")),
    })
}

fn process(bufin: impl BufRead) -> Result<u32> {
    let re1 = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)")?;
    let re2 = Regex::new(r"([0-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|orez)")?;
    let lines = parser::parse(bufin)?;
    Ok(lines
        .into_iter()
        .map(|line| {
            let first = re1
                .find(&line)
                .ok_or_else(|| eyre!("could not match re1"))?;
            let linerev = line.chars().rev().collect::<String>();
            let lastrev = re2
                .find(&linerev)
                .ok_or_else(|| eyre!("could not match re2"))?;
            let last = lastrev.as_str().chars().rev().collect::<String>();
            let first = readnum(first.as_str())?;
            let last = readnum(&last)?;
            let value = first * 10 + last;
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 281);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
