// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use std::io::{stdin, BufRead};
pub use std::time::Instant;

pub use color_eyre::eyre::eyre;
pub use color_eyre::Report;
pub use color_eyre::Result;

#[macro_use]
pub mod parser {
    pub use color_eyre::eyre::eyre;
    pub use color_eyre::Report;
    pub use color_eyre::Result;
    pub use combinator::all_consuming;
    pub use nom::branch;
    pub use nom::bytes::complete as bytes;
    pub use nom::character::complete as character;
    pub use nom::combinator;
    pub use nom::multi;
    pub use nom::Finish;
    pub use nom::IResult;
    pub use std::io::BufRead;

    #[macro_export]
    macro_rules! parse_with {
        ($parser:expr, $buf:ident) => {{
            let mut input = String::default();
            $buf.read_to_string(&mut input)?;
            let result = all_consuming($parser)(&input).finish();
            Ok(result.map_err(|e| eyre!("error reading input: {:?}", e))?.1)
        }};
    }
}

pub fn do_main<F: Fn() -> Result<T>, T: std::fmt::Display>(f: F) -> Result<()> {
    color_eyre::install()?;
    let start = Instant::now();
    println!("{}", f()?);
    println!("Elapsed: {}", humantime::Duration::from(start.elapsed()));
    Ok(())
}
