// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::fmt;
use std::str::FromStr;

pub const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Cell {
    Ok,
    Broken,
    Unknown,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Ok => write!(f, "."),
            Cell::Broken => write!(f, "#"),
            Cell::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Ok),
            '#' => Ok(Cell::Broken),
            '?' => Ok(Cell::Unknown),
            other => Err(eyre!("invalid spring {}", other)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Row(pub Vec<Cell>);

impl Row {
    pub fn subrow(&self, from: usize) -> Row {
        Row(self.0[from..].to_vec())
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.0 {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

impl FromStr for Row {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row(s
            .chars()
            .map(Cell::try_from)
            .collect::<Result<Vec<Cell>>>()?))
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#?")(input)?;
        Ok((input, Cell::try_from(c).unwrap()))
    }

    fn line(input: &str) -> IResult<&str, (Row, Vec<u32>)> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::space1(input)?;
        let (input, runs) = multi::separated_list1(bytes::tag(","), character::u32)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (Row(cells), runs)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<(Row, Vec<u32>)>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    assert_eq!(parser::parse(EXAMPLE.as_bytes())?.len(), 6);
    Ok(())
}

pub type Cache = std::collections::HashMap<(Row, Vec<u32>), usize>;

pub fn helper(
    cache: &mut Cache,
    row: &mut Row,
    irow: usize,
    record: Vec<u32>,
    currun: usize,
    _lastok: bool,
) -> usize {
    if irow == row.0.len() {
        let success = record.is_empty() || record == vec![0];
        return if success { 1 } else { 0 };
    }
    let key = (row.subrow(irow), record.clone());
    if let Some(result) = cache.get(&key) {
        return *result;
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
            helper(cache, row, irow + 1, newrecord, 0, true)
        }
        Cell::Broken => {
            if record.is_empty() || record[0] == 0 {
                // We can't have another broken
                return 0;
            }
            let mut newrecord = record.clone();
            newrecord[0] -= 1;
            helper(cache, row, irow + 1, newrecord, currun + 1, false)
        }
        Cell::Unknown => {
            row.0[irow] = Cell::Ok;
            let newrecord1 = record.clone();
            let oksum = helper(cache, row, irow, newrecord1, currun, _lastok);
            let brksum = if !record.is_empty() || record != vec![0] {
                row.0[irow] = Cell::Broken;
                let newrecord2 = record.clone();
                helper(cache, row, irow, newrecord2, currun, _lastok)
            } else {
                0
            };
            row.0[irow] = Cell::Unknown;
            cache.insert(key, oksum + brksum);
            oksum + brksum
        }
    }
}

pub fn calc_arrangements(row: &Row, record: &[u32]) -> usize {
    let mut mrow = row.clone();
    let rec = record.to_vec();
    let mut cache = Cache::new();
    helper(&mut cache, &mut mrow, 0, rec, 0, true)
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
