// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day20::*;

use std::collections::BTreeMap;

fn process(bufin: impl BufRead) -> Result<u64> {
    let modules = parser::parse(bufin)?;
    let mut sts = sts_init(modules);
    let broadcast_mname: Mname = "0".into();
    sts.insert(&broadcast_mname, ModState::default());
    // This is hard-coded from inspecting the graph:
    let mut targets: BTreeMap<Mname, Option<u64>> = [
        ("kc".into(), None),
        ("hd".into(), None),
        ("fl".into(), None),
        ("tb".into(), None),
    ]
    .into_iter()
    .collect();
    for button in 1..u64::MAX {
        let mut pulses = vec![(broadcast_mname, false, broadcast_mname)];
        while !pulses.is_empty() {
            let mut next_pulses = Vec::<(Mname, bool, Mname)>::new();
            for (src_mname, pulse, dst_mname) in pulses.into_iter() {
                let module = &modules[&dst_mname];
                let module_pulses =
                    eval(module, sts.get_mut(&dst_mname).unwrap(), src_mname, pulse);
                next_pulses.extend(module_pulses.into_iter());
            }
            pulses = next_pulses;
            for (src_mname, pulse, _dst_mname) in &pulses {
                if *pulse {
                    continue;
                }
                if targets.contains_key(src_mname) && targets[src_mname].is_none() {
                    targets.insert(*src_mname, Some(button));
                }
            }
            if targets.values().all(|v| v.is_some()) {
                // Also hard-coded: they all activate at button
                // presses that are prime numbers, so we can just
                // multiply:
                return Ok(targets.values().map(|opt| opt.unwrap()).product::<u64>());
            }
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
