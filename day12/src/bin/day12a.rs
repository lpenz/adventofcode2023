// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{stdin, BufRead};

use day12::*;

fn helper(row: &mut Row, irow: usize, record: Vec<u32>, currun: usize, _lastok: bool) -> usize {
    if irow == row.0.len() {
        let success = record.is_empty() || record == vec![0];
        return if success { 1 } else { 0 };
    }
    match row.0[irow] {
        Cell::Ok => {
            let mut newrecord = record.clone();
            if currun > 0 {
                // End run
                if newrecord[0] != 0 {
                    return 0;
                }
                newrecord.remove(0);
            }
            helper(row, irow + 1, newrecord, 0, true)
        }
        Cell::Broken => {
            if record.is_empty() || record[0] == 0 {
                // We can't have another broken
                return 0;
            }
            let mut newrecord = record.clone();
            newrecord[0] -= 1;
            helper(row, irow + 1, newrecord, currun + 1, false)
        }
        Cell::Unknown => {
            let mut sum = 0;
            row.0[irow] = Cell::Ok;
            let newrecord1 = record.clone();
            sum += helper(row, irow, newrecord1, currun, _lastok);
            row.0[irow] = Cell::Broken;
            let newrecord2 = record.clone();
            sum += helper(row, irow, newrecord2, currun, _lastok);
            row.0[irow] = Cell::Unknown;
            sum
        }
    }
}

fn calc_arrangements(row: &Row, record: &[u32]) -> usize {
    let mut mrow = row.clone();
    let rec = record.to_vec();
    helper(&mut mrow, 0, rec, 0, true)
}

#[test]
fn test_calc_arrangements_1() -> Result<()> {
    let row = "???.###".parse::<Row>()?;
    let record = vec![1, 1, 3];
    assert_eq!(calc_arrangements(&row, &record), 1);
    Ok(())
}

#[test]
fn test_calc_arrangements_2() -> Result<()> {
    let row = ".??..??...?##.".parse::<Row>()?;
    let record = vec![1, 1, 3];
    assert_eq!(calc_arrangements(&row, &record), 4);
    Ok(())
}

#[test]
fn test_calc_arrangements_3() -> Result<()> {
    let row = "?#?#?#?#?#?#?#?".parse::<Row>()?;
    let record = vec![1, 3, 1, 6];
    assert_eq!(calc_arrangements(&row, &record), 1);
    Ok(())
}

#[test]
fn test_calc_arrangements_4() -> Result<()> {
    // let row = "????.#...#...".parse::<Row>()?;
    let row = "####.#...#...".parse::<Row>()?;
    let record = vec![4, 1, 1];
    assert_eq!(calc_arrangements(&row, &record), 1);
    Ok(())
}

#[test]
fn test_calc_arrangements_5() -> Result<()> {
    let row = "????.######..#####.".parse::<Row>()?;
    let record = vec![1, 6, 5];
    assert_eq!(calc_arrangements(&row, &record), 4);
    Ok(())
}

#[test]
fn test_calc_arrangements_6() -> Result<()> {
    let row = "?###????????".parse::<Row>()?;
    let record = vec![3, 2, 1];
    assert_eq!(calc_arrangements(&row, &record), 10);
    Ok(())
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    Ok(input
        .into_iter()
        .map(|entry| calc_arrangements(&entry.0, &entry.1))
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 21);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
