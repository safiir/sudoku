# Sudoku Solver

A fast Sudoku solver written in Rust. It handles boards of any order — `1×1`,
`4×4`, `9×9`, `16×16`, `25×25` — using bitmask constraint propagation combined
with a minimum-remaining-values backtracking search.

## Features

- **Any order.** Works on any `N×N` board whose side is a perfect square, up to
  `25×25` (limited by the 32-bit constraint masks).
- **Bitmask constraints.** Each row, column, and box tracks its used digits as a
  single integer bitmask, so candidate lookups are plain bitwise operations.
- **Constraint propagation.** Naked singles are filled in repeatedly before any
  search, pruning the tree aggressively.
- **MRV backtracking.** When a guess is needed, the solver branches on the empty
  cell with the fewest candidates first.
- **All solutions.** Returns every valid solution, not just the first.
- **Colored output.** Solved boards are printed to the terminal with per-box
  coloring.

## How it works

1. **Encode.** For every row, column, and box, the digits already present are
   packed into a bitmask (bit `d-1` set means digit `d` is used).
2. **Propagate** (`reasoning`). For each empty cell, `available = full & !(row |
   col | box)`. If exactly one bit remains (`count_ones() == 1`), that digit is
   forced; the cell is filled and the masks updated. This repeats until no more
   cells can be resolved.
3. **Search** (`backtrack`). If empty cells remain, pick the one with the fewest
   candidates (MRV) and recurse on each candidate. The masks are threaded through
   the recursion and updated incrementally rather than rebuilt at every node.

## Build & Run

```sh
cargo run --release
```

This solves the puzzle defined in `main` and prints the solution along with the
elapsed time. A hard `9×9` board is solved in a few milliseconds in release mode.

## Usage

The solver is exposed as `Sudoku::solve`, which takes the board as a
`Vec<Vec<char>>` where `'.'` marks an empty cell:

```rust
let board = [
  "8........",
  "..36.....",
  ".7..9.2..",
  ".5...7...",
  "....457..",
  "...1...3.",
  "..1....68",
  "..85...1.",
  ".9....4..",
]
.iter()
.map(|row| row.chars().collect::<Vec<_>>())
.collect::<Vec<_>>();

match Sudoku::solve(board) {
  Some(solutions) => println!("{} solution(s) found", solutions.len()),
  None => println!("no solution"),
}
```

For boards larger than `9×9`, cells use base-`(N+1)` digits (`0-9`, then `a`,
`b`, …), matching the terminal output.
