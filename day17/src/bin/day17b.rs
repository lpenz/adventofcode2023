// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day17::*;

fn process(size: u16, bufin: impl BufRead) -> Result<u32> {
    let input = parser::parse(bufin)?;
    let gheat = Grid::try_from(input)?;
    solve(size, gheat, |st, dir| {
        true
            // Can't go back:
            && st.lastdir != Some(-dir)
            // Must go at least 4 spaces:
            && (st.lastdir.is_none() || st.lastdir == Some(dir) || st.dircount >= 4)
            // And not more than 10:
            && (st.lastdir != Some(dir) || st.dircount < 10)
    })
}

#[test]
fn test() -> Result<()> {
    let start = std::time::Instant::now();
    assert_eq!(process(13, EXAMPLE.as_bytes())?, 94);
    println!("Elapsed: {}", elapsed(&start));
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(141, stdin().lock()))
}
