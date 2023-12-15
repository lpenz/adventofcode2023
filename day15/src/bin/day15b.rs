// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;
use std::io::{stdin, BufRead};

use day15::*;

pub type FocalLen = u32;

#[test]
fn test_hash() {
    assert_eq!(hash_str("HASH"), 52);
}

#[derive(Default)]
pub struct Lbox {
    order: Vec<Label>,
    pos: HashMap<Label, usize>,
    lens: HashMap<Label, FocalLen>,
}

impl Lbox {
    pub fn pos_fix(&mut self) {
        self.pos = self
            .order
            .iter()
            .enumerate()
            .map(|(a, b)| (b.clone(), a))
            .collect();
    }
}

fn process(bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let mut boxes = HashMap::<usize, Lbox>::default();
    for step in &input {
        let ibox = hash_str(&step.label.0);
        match step.op {
            Op::Del => {
                if let Some(lbox) = boxes.get_mut(&ibox) {
                    lbox.lens.remove(&step.label);
                    if let Some(i) = lbox.pos.remove(&step.label) {
                        lbox.order.remove(i);
                    }
                    lbox.pos_fix();
                }
            }
            Op::Focus(fl) => {
                let lbox = boxes.entry(ibox).or_default();
                if let Some(lens) = lbox.lens.get_mut(&step.label) {
                    *lens = fl;
                } else {
                    lbox.lens.insert(step.label.clone(), fl);
                    lbox.pos.insert(step.label.clone(), lbox.order.len());
                    lbox.order.push(step.label.clone());
                }
            }
        };
    }
    Ok(boxes
        .into_iter()
        .map(|(ib, b)| {
            b.order
                .iter()
                .enumerate()
                .map(|(il, label)| (ib + 1) * (il + 1) * (*b.lens.get(label).unwrap() as usize))
                .sum::<usize>()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 145);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", process(stdin().lock())?);
    Ok(())
}
