// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day19::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Range([(u64, u64); 4]);

impl Range {
    pub const DEFAULT: Range = Range([(1, 4000), (1, 4000), (1, 4000), (1, 4000)]);

    pub fn split(&self, cond: &Cond) -> (Range, Range) {
        let par = cond.par;
        let mut matching = *self;
        let mut nonmatch = *self;
        if cond.op == Op::Lt {
            matching.0[par as usize].1 = cond.value - 1;
            nonmatch.0[par as usize].0 = cond.value;
        } else {
            // if cond.op == Op.Gt
            matching.0[par as usize].0 = cond.value + 1;
            nonmatch.0[par as usize].1 = cond.value;
        }
        (matching, nonmatch)
    }

    pub fn size(&self) -> u64 {
        self.0
            .iter()
            .map(|(low, high)| high - low + 1_u64)
            .product()
    }
}

fn weval(workflows: &HashMap<Wname, Workflow>, mut range: Range, wname: Wname) -> u64 {
    let workflow = &workflows[&wname];
    let mut result = 0;
    for rule in &workflow.rules {
        if let Some(cond) = &rule.cond {
            let (matching, nonmatch) = range.split(cond);
            if let Some(next_wname) = rule.act.get_wname() {
                result += weval(workflows, matching, next_wname);
            } else if rule.act == Action::Accept {
                result += matching.size();
            }
            range = nonmatch;
        } else if let Some(next_wname) = rule.act.get_wname() {
            return result + weval(workflows, range, next_wname);
        } else if rule.act == Action::Accept {
            return result + range.size();
        }
    }
    result
}

fn process(bufin: impl BufRead) -> Result<u64> {
    let (workflows, _) = parser::parse(bufin)?;
    let workflows = workflows
        .into_iter()
        .map(|w| (w.wname, w))
        .collect::<HashMap<_, _>>();
    let range = Range::DEFAULT;
    Ok(weval(&workflows, range, Wname::try_from("in").unwrap()))
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 167409079868000);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
