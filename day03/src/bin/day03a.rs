// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use sqrid::Qr;

use day03::*;

type Sqrid = sqrid::sqrid_create!(140, 140, true);
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, Cell);

fn process(bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let mut grid = Grid::default();
    for (y, line) in input.into_iter().enumerate() {
        for (x, cell) in line.into_iter().enumerate() {
            let qa = Qa::try_from((x as u16, y as u16))?;
            grid[qa] = cell;
        }
    }
    let mut numbers = vec![];
    // Look for symbols:
    for qa_symbol in Qa::iter() {
        if !matches!(grid[qa_symbol], Cell::Symbol(_)) {
            continue;
        }
        // Check adjacencies, with diagonals:
        for qr in Qr::iter::<true>() {
            let Ok(qa_adj) = qa_symbol + qr else { continue };
            if !matches!(grid[qa_adj], Cell::Digit(_)) {
                continue;
            }
            // Found a digit, go left:
            let mut qa_start = qa_adj;
            while let Ok(qa) = qa_start + Qr::W {
                if !matches!(grid[qa], Cell::Digit(_)) {
                    break;
                }
                qa_start = qa;
            }
            // We are at the start of the number, pick up the digits
            let mut number_str = format!("{}", grid[qa_start].digit()?);
            let mut qa_digit = qa_start;
            grid[qa_digit] = Cell::Empty;
            while let Ok(qa) = qa_digit + Qr::E {
                if !matches!(grid[qa], Cell::Digit(_)) {
                    break;
                }
                number_str.push(grid[qa].digit()?);
                grid[qa] = Cell::Empty;
                qa_digit = qa;
            }
            let number = number_str
                .parse::<u32>()
                .map_err(|_| eyre!("invalid number {}", number_str))?;
            numbers.push(number);
        }
    }
    Ok(numbers.into_iter().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 4361);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
