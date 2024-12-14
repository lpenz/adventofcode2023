// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use sqrid::postrait::PosT;

pub const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

pub use sqrid::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Slope(Dir),
}

impl TryFrom<char> for Cell {
    type Error = Report;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            '^' => Ok(Cell::Slope(Dir::N)),
            '>' => Ok(Cell::Slope(Dir::E)),
            'v' => Ok(Cell::Slope(Dir::S)),
            '<' => Ok(Cell::Slope(Dir::W)),
            other => Err(eyre!("invalid cell {}", other)),
        }
    }
}

impl From<&Cell> for char {
    fn from(c: &Cell) -> Self {
        match c {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Slope(Dir::N) => '^',
            Cell::Slope(Dir::E) => '>',
            Cell::Slope(Dir::S) => 'v',
            Cell::Slope(Dir::W) => '<',
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn cell(input: &str) -> IResult<&str, Cell> {
        let (input, c) = character::one_of(".#^>v<")(input)?;
        Ok((input, Cell::try_from(c).unwrap()))
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
    assert_eq!(input.len(), 23);
    assert_eq!(input[0].len(), 23);
    Ok(())
}

pub type Sqrid = sqrid::sqrid_create!(141, 141, false);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Node {
    pub pos: Pos,
    pub children: Vec<(Pos, usize)>,
}

impl Node {
    pub fn new(pos: Pos) -> Self {
        Node {
            pos,
            children: vec![],
        }
    }
}

fn go<const SLOPES: bool>(size: u16, grid: &Grid, pos: Pos, dir: Dir) -> Option<Pos> {
    (pos + dir).ok().filter(|newpos| {
        let newpos_t = newpos.tuple();
        newpos_t.0 < size
            && newpos_t.1 < size
            && newpos != &pos
            && match grid[newpos] {
                Cell::Empty => true,
                Cell::Slope(d) => !SLOPES || d == dir,
                _ => false,
            }
    })
}

pub fn is_node<const SLOPES: bool>(size: u16, grid: &Grid, pos: Pos) -> bool {
    let pos_t = pos.tuple();
    if grid[pos] == Cell::Wall || pos_t.0 >= size || pos_t.1 >= size {
        false
    } else {
        let num_children_noslopes = Dir::iter::<false>()
            .filter(|dir| go::<false>(size, grid, pos, *dir).is_some())
            .count();
        if !SLOPES {
            num_children_noslopes != 2
        } else {
            let num_children_slopes = Dir::iter::<false>()
                .filter(|dir| go::<true>(size, grid, pos, *dir).is_some())
                .count();
            num_children_slopes != 2 || num_children_noslopes != 2
        }
    }
}

pub fn node_craft<const SLOPES: bool>(size: u16, grid: &Grid, pos: Pos) -> Node {
    let visited0 = [pos].into_iter().collect::<HashSet<Pos>>();
    Node {
        pos,
        children: Dir::iter::<false>()
            .filter(|dir| go::<SLOPES>(size, grid, pos, *dir).is_some())
            .map(|child_dir| {
                let mut newpos = pos;
                let mut visited = visited0.clone();
                let mut next = vec![(newpos + child_dir).unwrap()];
                let mut steps = 0;
                while next.len() == 1 {
                    steps += 1;
                    newpos = next[0];
                    if is_node::<SLOPES>(size, grid, newpos) {
                        break;
                    }
                    visited.insert(newpos);
                    next = Dir::iter::<false>()
                        .filter_map(|dir| {
                            go::<SLOPES>(size, grid, newpos, dir).filter(|p| !visited.contains(p))
                        })
                        .collect::<Vec<_>>();
                }
                (newpos, steps)
            })
            .collect::<Vec<(Pos, usize)>>(),
    }
}

pub fn dfs(
    nodes: &HashMap<Pos, Node>,
    pos: Pos,
    visited: &HashSet<Pos>,
    end: Pos,
    steps: usize,
) -> usize {
    if pos == end {
        return steps;
    }
    if visited.contains(&pos) {
        return 0;
    }
    let mut newvisited = visited.clone();
    newvisited.insert(pos);
    let node = nodes.get(&pos).unwrap();
    node.children
        .iter()
        .map(|(newpos, newsteps)| dfs(nodes, *newpos, &newvisited, end, steps + *newsteps))
        .max()
        .unwrap_or(0)
}

pub fn solve<const SLOPES: bool>(size: u16, bufin: impl BufRead) -> Result<usize> {
    let input = parser::parse(bufin)?;
    let grid = Grid::try_from(input)?;
    let nodes = Pos::iter()
        .filter(|p| is_node::<SLOPES>(size, &grid, *p))
        .map(|p| (p, node_craft::<SLOPES>(size, &grid, p)))
        .collect::<HashMap<_, _>>();
    let start = Pos::new(1, 0)?;
    let end = Pos::new(size - 2, size - 1)?;
    let visited = Default::default();
    Ok(dfs(&nodes, start, &visited, end, 0))
}
