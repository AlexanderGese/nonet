use crate::board::Board;
use crate::rng::Rng;
use crate::solver::{count_solutions, solve_random};

pub struct Puzzle {
    pub board: Board,
    pub solution: Board,
}

pub fn clues_for(difficulty: &str) -> usize {
    match difficulty {
        "easy" => 40,
        "hard" => 27,
        "expert" => 23,
        _ => 32, // medium
    }
}

pub fn generate(rng: &mut Rng, target_clues: usize) -> Puzzle {
    // start from a random finished grid, then knock cells out one by one
    let mut full = Board::empty();
    solve_random(&mut full, rng);
    let solution = full;

    let mut puzzle = full;
    let mut order: Vec<usize> = (0..81).collect();
    rng.shuffle(&mut order);

    let mut clues = 81;
    for &idx in &order {
        if clues <= target_clues {
            break;
        }
        let saved = puzzle.cells[idx];
        puzzle.cells[idx] = 0;
        let mut test = puzzle;
        if count_solutions(&mut test, 2) == 1 {
            clues -= 1;
        } else {
            puzzle.cells[idx] = saved; // taking this one out gives >1 solution, keep it
        }
    }

    Puzzle { board: puzzle, solution }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve;

    #[test]
    fn generated_are_unique_and_match_solution() {
        for seed in 1..6u64 {
            let mut r = Rng::new(seed);
            let p = generate(&mut r, 30);
            let mut t = p.board;
            assert_eq!(count_solutions(&mut t, 2), 1);
            let mut s = p.board;
            assert!(solve(&mut s));
            assert_eq!(s.cells, p.solution.cells);
        }
    }
}
