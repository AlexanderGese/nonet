# nonet

A Sudoku solver and generator with a visual terminal UI. Watch the
backtracking search fill the grid and walk back out of dead ends in real time.

There's a live version in the browser too (same solver, compiled to WebAssembly):
**https://alexandergese.github.io/nonet/**

## What it does

- **Solves** any valid puzzle. The search uses the MRV heuristic — it always
  branches on the cell with the fewest remaining candidates — so even the
  "hard" boards finish basically instantly.
- **Generates** puzzles with a *unique* solution. It fills a random complete
  grid, then digs cells out one at a time, only keeping a removal if the puzzle
  still has exactly one answer.
- **Shows its work.** The default mode is an animated TUI: givens in white,
  the solver's own placements in green, the current cell highlighted, and
  backtracks flashing red.

## Use it

```
nonet                       # animated solve of a fresh hard puzzle
nonet -d expert             # pick easy | medium | hard | expert
nonet generate -d hard      # just print a puzzle
nonet solve <81 chars>      # solve one ('.' or 0 for blanks)
```

In the TUI: `space` play/pause · `n` new · `r` reset · `s` solve instantly ·
`+`/`-` speed · `1`–`4` difficulty · `q` quit.

## Build

```
cargo build --release
cargo test
```

## License

MIT
