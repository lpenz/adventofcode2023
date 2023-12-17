// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

pub type Cell = u32;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, val) = character::one_of("0123456789")(input)?;
        Ok((input, val.to_digit(10).unwrap()))
    }

    fn line(input: &str) -> IResult<&str, Vec<Cell>> {
        let (input, cells) = multi::many1(cell)(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, cells))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Vec<Vec<Cell>>> {
        aoc::parse_with!(multi::many1(line), bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.len(), 13);
    assert_eq!(input[0].len(), 13);
    Ok(())
}

pub use sqrid::Dir;
pub type Sqrid = sqrid::sqrid_create!(141, 141, false);
// pub type Sqrid = sqrid::sqrid_create!(13, 13, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, u32);

pub type Griddir = sqrid::grid_create!(Sqrid, String);

pub fn path_debug(_size: u16, gheat: &Grid, path: &[Dir]) {
    let mut gheatacum = Grid::default();
    let mut pos = Pos::TOP_LEFT;
    let mut heat = 0;
    let mut gdir = Griddir::default();
    for dir in path {
        gdir[pos] = dir.name_utf8().to_string();
        pos = (pos + dir).unwrap();
        heat += gheat[pos];
        gheatacum[pos] = heat;
    }
    eprintln!("{:1}", gdir);
    eprintln!("{:>4}", gheatacum);
}
