![AoC](https://img.shields.io/badge/AoC%20%E2%AD%90-50-yellow)
[![CI](https://github.com/lpenz/adventofcode2023/workflows/CI/badge.svg)](https://github.com/lpenz/adventofcode2023/actions)
[![coveralls](https://coveralls.io/repos/github/lpenz/adventofcode2023/badge.svg?branch=main)](https://coveralls.io/github/lpenz/adventofcode2023?branch=main)

# adventofcode2023

Code for the 2023 puzzles at https://adventofcode.com/2023/


## Noteworthy days (spoiler alert!)

Some interesting things that happened on specific days:

- Day 05b: used `Range{ ini, end }` to represent sets of numbers
  (which was not necessary).
- Day 20b: had to look at the graph to figure out the "counter result
  modules".
- Day 24b: used [z3] in rust.
- Day 25a: graph min cut problem; did it with multiple Dijsktra, maybe
  try [Stoer-Wagner] in the future.


<table><tr>
<td><a href="https://github.com/lpenz/adventofcode2022">:arrow_left: 2022</td>
<td><a href="https://github.com/lpenz/adventofcode2024">2024 :arrow_right:</td>
</tr></table>

[z3]: https://docs.rs/z3/latest/z3/
[Stoer-Wagner]: https://scholar.google.com/scholar?cluster=10111487970680388034
