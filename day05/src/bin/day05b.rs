// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use rayon::prelude::*;
use std::io::{stdin, BufRead};

use day05::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub ini: i64,
    pub end: i64,
}

impl Range {
    pub fn new(ini: i64, end: i64) -> Range {
        Range { ini, end }
    }

    pub fn subtract(&self, other: &Range) -> Vec<Range> {
        if self.end < other.ini || other.end < self.ini {
            // disjunction
            vec![*self]
        } else if self.ini < other.ini && other.end < self.end {
            // contains other
            vec![
                Range::new(self.ini, other.ini - 1),
                Range::new(other.end + 1, self.end),
            ]
        } else if other.ini <= self.ini && self.end <= other.end {
            // contained in other
            vec![]
        } else if self.ini < other.ini && self.end <= other.end {
            // overlap, self first
            vec![Range::new(self.ini, other.ini - 1)]
        } else if other.ini <= self.ini && other.end < self.end {
            // overlap, other first
            vec![Range::new(other.end + 1, self.end)]
        } else {
            panic!("unknown relation between {:?} and {:?}", self, other);
        }
    }

    pub fn intersection(&self, other: &Range) -> Option<Range> {
        if self.end < other.ini || other.end < self.ini {
            // disjunction
            None
        } else if self.ini < other.ini && other.end < self.end {
            // contains other
            // Some(Range::new(other.ini, other.end))
            Some(Range::new(other.ini, other.end))
        } else if other.ini <= self.ini && self.end <= other.end {
            // contained in other
            Some(Range::new(self.ini, self.end))
        } else if self.ini < other.ini && self.end <= other.end {
            // overlap, self first
            Some(Range::new(other.ini, self.end))
        } else if other.ini <= self.ini && other.end < self.end {
            // overlap, other first
            Some(Range::new(self.ini, other.end))
        } else {
            panic!("unknown relation between {:?} and {:?}", self, other);
        }
    }

    pub fn map(&self, other: &Range, dest: i64) -> Range {
        Range::new(dest - other.ini + self.ini, dest - other.ini + self.end)
    }

    pub fn intersection_map(&self, other: &Range, dest: i64) -> Option<Range> {
        self.intersection(other).map(|r| r.map(other, dest))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VRange(pub Vec<Range>);

impl VRange {
    pub fn new(ini: i64, end: i64) -> VRange {
        VRange(vec![Range::new(ini, end)])
    }

    pub fn subtract(&self, other: &VRange) -> VRange {
        VRange(other.0.iter().fold(self.0.clone(), |left, sub| {
            left.into_iter()
                .flat_map(|l: Range| l.subtract(sub))
                .collect()
        }))
    }

    pub fn intersection(&self, other: &VRange) -> VRange {
        VRange(
            self.0
                .iter()
                .flat_map(|r| other.0.iter().flat_map(|o| r.intersection(o)))
                .collect(),
        )
    }

    pub fn intersection_map(&self, other: &VRange, dest: i64) -> VRange {
        VRange(
            self.0
                .iter()
                .flat_map(|r| other.0.iter().flat_map(|o| r.intersection_map(o, dest)))
                .collect(),
        )
    }
}

#[test]
fn test_vrange() {
    // empty
    assert_eq!(
        VRange::new(2, 4).subtract(&VRange::default()),
        VRange::new(2, 4)
    );
    // disjunction:
    assert_eq!(
        VRange::new(2, 4).intersection_map(&VRange::new(5, 6), 9),
        VRange::default()
    );
    assert_eq!(
        VRange::new(2, 4).subtract(&VRange::new(5, 6)),
        VRange::new(2, 4)
    );
    // equal:
    assert_eq!(
        VRange::new(2, 4).intersection_map(&VRange::new(2, 4), 9),
        VRange::new(9, 11)
    );
    assert_eq!(
        VRange::new(2, 4).subtract(&VRange::new(2, 4)),
        VRange::default()
    );
    // contained in src:
    assert_eq!(
        VRange::new(2, 4).intersection_map(&VRange::new(1, 5), 9),
        VRange::new(10, 12)
    );
    assert_eq!(
        VRange::new(2, 4).subtract(&VRange::new(1, 5)),
        VRange::default()
    );
    // contains src:
    assert_eq!(
        VRange::new(0, 6).intersection_map(&VRange::new(2, 4), 9),
        VRange::new(9, 11)
    );
    assert_eq!(
        VRange::new(0, 6).subtract(&VRange::new(2, 4)),
        VRange(vec![Range::new(0, 1), Range::new(5, 6),])
    );
    // overlap seed_ini first
    assert_eq!(
        VRange::new(0, 4).intersection_map(&VRange::new(2, 6), 9),
        VRange::new(9, 11)
    );
    assert_eq!(
        VRange::new(0, 4).subtract(&VRange::new(2, 6)),
        VRange::new(0, 1)
    );
    // overlap src_ini first
    assert_eq!(
        VRange::new(2, 6).intersection_map(&VRange::new(0, 4), 9),
        VRange(vec![Range::new(11, 13)])
    );
    assert_eq!(
        VRange::new(2, 6).subtract(&VRange::new(0, 4)),
        VRange::new(5, 6)
    );
}

fn process(bufin: impl BufRead) -> Result<i64> {
    let input = parser::parse(bufin)?;
    let (seeds, almanac) = input;
    // Convert almanac and seeds to the VRange type:
    let almanac = almanac
        .into_iter()
        .map(|a| {
            a.into_iter()
                .map(|(dest, ini, len)| {
                    (dest as i64, VRange::new(ini as i64, (ini + len - 1) as i64))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let seeds = seeds
        .chunks_exact(2)
        .map(|seedrange| {
            let &[ini, len] = seedrange else {
                panic!("invalid range")
            };
            let end: i64 = ini as i64 + len as i64 - 1;
            VRange::new(ini as i64, end)
        })
        .collect::<Vec<_>>();
    seeds
        .into_par_iter()
        .map(|seed| {
            almanac.iter().fold(seed, |current, map| {
                let mut next = VRange::default();
                let mut left = current.clone();
                for (dest, entry) in map {
                    let intersection = current.intersection(entry);
                    let mapped = current.intersection_map(entry, *dest);
                    left = left.subtract(&intersection);
                    // Passthrough the ones that are left:
                    next.0.extend(&mapped.0);
                }
                next.0.extend(&left.0);
                next
            })
        })
        .filter_map(|vrange| vrange.0.into_iter().map(|r| r.ini).min())
        .min()
        .ok_or_else(|| eyre!("no seed found"))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 46);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
