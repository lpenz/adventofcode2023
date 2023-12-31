// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day17::*;

fn process(size: u16, bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let gheat = Grid::try_from(input)?;
    solve(size, gheat, |st, dir| {
        st.lastdir != Some(-dir) && (st.lastdir != Some(dir) || st.dircount < 3)
    })
}

#[test]
fn test() -> Result<()> {
    let start = std::time::Instant::now();
    assert_eq!(process(13, EXAMPLE.as_bytes())?, 102);
    println!("Elapsed: {}", elapsed(&start));
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(141, stdin().lock()))
}
