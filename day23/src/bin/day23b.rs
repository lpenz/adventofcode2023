// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day23::*;

fn process(size: u16, bufin: impl BufRead) -> Result<usize> {
    solve::<false>(size, bufin)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(23, EXAMPLE.as_bytes())?, 154);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|| process(141, stdin().lock()))
}
