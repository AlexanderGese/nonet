mod board;
mod generator;
mod rng;
mod solver;

use board::Board;
use rng::Rng;
use solver::Step;
use wasm_bindgen::prelude::*;

// make a puzzle, return the 81-char line ('.' = empty, given = the rest)
#[wasm_bindgen]
pub fn gen(difficulty: &str, seed: u64) -> String {
    let mut r = Rng::new(seed);
    let p = generator::generate(&mut r, generator::clues_for(difficulty));
    p.board.to_line()
}

// the search trace as compact json: [[idx,val],...]  val 0 = backtrack/erase.
// the page replays these to animate the solve.
#[wasm_bindgen]
pub fn trace(puzzle: &str) -> String {
    let Some(mut b) = Board::parse(puzzle) else {
        return "[]".into();
    };
    let mut steps = Vec::new();
    solver::solve_trace(&mut b, &mut steps);

    let mut out = String::from("[");
    for (k, s) in steps.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        match s {
            Step::Place(i, v) => out.push_str(&format!("[{i},{v}]")),
            Step::Erase(i) => out.push_str(&format!("[{i},0]")),
        }
    }
    out.push(']');
    out
}

// straight solution (no animation) - returns the solved 81-char line
#[wasm_bindgen]
pub fn solve(puzzle: &str) -> String {
    let Some(mut b) = Board::parse(puzzle) else {
        return String::new();
    };
    if solver::solve(&mut b) {
        b.to_line()
    } else {
        String::new()
    }
}
