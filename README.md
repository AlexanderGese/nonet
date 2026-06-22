# 🧩 nonet

[![crates.io](https://img.shields.io/crates/v/nonet.svg)](https://crates.io/crates/nonet)

A Sudoku solver and generator with a visual terminal UI. Watch the backtracking
search fill the grid and walk back out of dead ends in real time.

**▶ Try it live (no install): https://alexandergese.github.io/nonet/**

## Features

- **Solves** any valid puzzle. The search uses the MRV heuristic — it always
  branches on the cell with the fewest remaining candidates — so even the
  "world's hardest" boards finish basically instantly.
- **Generates** puzzles with a *unique* solution: fill a random complete grid,
  then dig cells out one at a time, only keeping a removal if the puzzle still
  has exactly one answer.
- **Shows its work.** The default mode is an animated TUI — givens in white, the
  solver's own placements in green, the current cell highlighted, backtracks
  flashing red.
- **No dependencies in the core** — own xorshift RNG, own candidate bit-masking.

## Install

From [crates.io](https://crates.io/crates/nonet):

```sh
cargo install nonet
```

That puts a `nonet` binary on your `PATH`. You'll need the Rust toolchain first;
if you don't have it, install it once with [rustup](https://rustup.rs):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Don't want to install anything? The
[web version](https://alexandergese.github.io/nonet/) runs entirely in the
browser, or you can [build from source](#build).

## Usage

```sh
nonet                       # animated solve of a fresh hard puzzle (default)
nonet -d expert             # pick easy | medium | hard | expert
nonet generate -d hard      # just print a puzzle
nonet solve <81 chars>      # solve one ('.' or 0 for blanks)
nonet demo                  # generate + solve, printed
```

In the TUI: `space` play/pause · `n` new · `r` reset · `s` solve instantly ·
`+`/`-` speed · `1`–`4` difficulty · `q` quit.

## How it works

The solver is plain backtracking, but it never guesses blindly: at each step it
scans for the empty cell with the **fewest legal candidates** (minimum remaining
values) and branches there. Candidates are tracked as a bitmask of the
row/column/box, so checking a cell is a few bit operations. That ordering alone
collapses the search tree enough that pathological "17-clue" puzzles solve in
microseconds.

The generator reuses the same solver: it builds a random finished grid, shuffles
the 81 cells, and removes them one by one — after each removal it re-counts the
solutions (stopping at 2) and puts the cell back if the board is no longer
unique.

## Build

```sh
cargo build --release
cargo test
```

Built with `ratatui` + `crossterm` for the UI and `clap` for the CLI.

## License

MIT
