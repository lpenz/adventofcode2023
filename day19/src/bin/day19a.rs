// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day19::*;

use std::collections::HashMap;

fn process(bufin: impl BufRead) -> Result<u64> {
    let (workflows, parts) = parser::parse(bufin)?;
    let workflows = workflows
        .into_iter()
        .map(|w| (w.wname, w))
        .collect::<HashMap<_, _>>();
    let mut totals: [u64; 4] = Default::default();
    for part in parts {
        let mut wname = Some(Wname::try_from("in").unwrap());
        while let Some(current) = wname {
            let workflow = &workflows[&current];
            if let Some(act) = workflow.eval(&part) {
                if act == Action::Accept {
                    for par in Par::ALL {
                        totals[par as usize] += part.get(par);
                    }
                }
                wname = act.get_wname();
            }
        }
    }
    Ok(totals.into_iter().sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE.as_bytes())?, 19114);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
