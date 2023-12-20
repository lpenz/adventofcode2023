// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day20::*;

use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct ModState {
    state: bool,
    // Conjunction memory:
    memory: BTreeMap<Mname, bool>,
}

fn eval<'a>(
    module: &'a Module,
    msts: &'a mut ModState,
    src_mname: Mname,
    pulse: bool,
) -> Box<dyn Iterator<Item = (Mname, bool, Mname)> + 'a> {
    match module.mtype {
        Mtype::Broadcast => Box::new(
            module
                .dsts
                .iter()
                .copied()
                .map(move |d| (module.mname, pulse, d)),
        ),
        Mtype::FlipFlop => {
            if !pulse {
                msts.state = !msts.state;
                Box::new(
                    module
                        .dsts
                        .iter()
                        .copied()
                        .map(|d| (module.mname, msts.state, d)),
                )
            } else {
                Box::new(std::iter::empty())
            }
        }
        Mtype::Conjunct => {
            msts.memory.insert(src_mname, pulse);
            // eprintln!("{} memory {:?}", module.mname.0, msts.memory);
            let pulse = !msts.memory.values().all(|v| *v);
            Box::new(
                module
                    .dsts
                    .iter()
                    .copied()
                    .map(move |d| (module.mname, pulse, d)),
            )
        }
        Mtype::None => Box::new(std::iter::empty()),
    }
}

fn process(bufin: impl BufRead) -> Result<u64> {
    let modules = parser::parse(bufin)?;
    let mut sts = BTreeMap::new();
    for mname in modules.keys() {
        sts.insert(mname, ModState::default());
    }
    for src_mname in modules.keys() {
        let src_module = &modules[src_mname];
        for dst_mname in &src_module.dsts {
            let dst_module = &modules[&dst_mname];
            if dst_module.mtype == Mtype::Conjunct {
                let conj = sts.get_mut(dst_mname).unwrap();
                conj.memory.insert(*src_mname, false);
            }
        }
    }
    let broadcast_mname: Mname = "0".into();
    sts.insert(&broadcast_mname, ModState::default());
    let mut total_low = 0_u64;
    let mut total_high = 0_u64;
    for _i in 0..1000 {
        let mut pulses = vec![(broadcast_mname, false, broadcast_mname)];
        while !pulses.is_empty() {
            // eprintln!("i {} pulses {:?}", i, pulses);
            let (low, high) = pulses.iter().fold((0, 0), |(low, high), (_, p, _)| {
                (low + if !p { 1 } else { 0 }, high + if *p { 1 } else { 0 })
            });
            total_low += low;
            total_high += high;
            let mut next_pulses = Vec::<(Mname, bool, Mname)>::new();
            for (src_mname, pulse, dst_mname) in pulses.into_iter() {
                let module = &modules[&dst_mname];
                let module_pulses =
                    eval(module, sts.get_mut(&dst_mname).unwrap(), src_mname, pulse);
                next_pulses.extend(module_pulses.into_iter());
            }
            pulses = next_pulses;
        }
    }
    Ok(total_low * total_high)
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(process(EXAMPLE1.as_bytes())?, 32000000);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(process(EXAMPLE2.as_bytes())?, 11687500);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(stdin().lock()))
}
