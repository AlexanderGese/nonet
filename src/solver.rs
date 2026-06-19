use crate::board::Board;
use crate::rng::Rng;

const ALL: u16 = 0x3FE; // bits 1..9

// pick the empty cell with the fewest options to branch on next. mask 0 means
// that cell is stuck (dead branch); None means the grid is full.
fn next_cell(b: &Board) -> Option<(usize, u16)> {
    let mut best = None;
    let mut best_n = 99;
    for idx in 0..81 {
        if b.cells[idx] != 0 {
            continue;
        }
        let mask = !b.used_mask(idx) & ALL;
        let n = mask.count_ones();
        if n == 0 {
            return Some((idx, 0));
        }
        if (n as i32) < best_n {
            best = Some((idx, mask));
            best_n = n as i32;
            if n == 1 {
                break;
            }
        }
    }
    best
}

pub fn solve(b: &mut Board) -> bool {
    match next_cell(b) {
        None => true,
        Some((_, 0)) => false,
        Some((idx, mask)) => {
            for v in 1..=9u8 {
                if mask & (1 << v) != 0 {
                    b.cells[idx] = v;
                    if solve(b) {
                        return true;
                    }
                    b.cells[idx] = 0;
                }
            }
            false
        }
    }
}

// same idea but visits candidates in random order, so we get a different full
// grid every time. the generator builds its solutions with this.
pub fn solve_random(b: &mut Board, rng: &mut Rng) -> bool {
    match next_cell(b) {
        None => true,
        Some((_, 0)) => false,
        Some((idx, mask)) => {
            let mut vals: Vec<u8> = (1..=9u8).filter(|v| mask & (1 << v) != 0).collect();
            rng.shuffle(&mut vals);
            for v in vals {
                b.cells[idx] = v;
                if solve_random(b, rng) {
                    return true;
                }
                b.cells[idx] = 0;
            }
            false
        }
    }
}

// count up to `limit` solutions - pass 2 when all we care about is "is it unique"
pub fn count_solutions(b: &mut Board, limit: usize) -> usize {
    match next_cell(b) {
        None => 1,
        Some((_, 0)) => 0,
        Some((idx, mask)) => {
            let mut total = 0;
            for v in 1..=9u8 {
                if mask & (1 << v) != 0 {
                    b.cells[idx] = v;
                    total += count_solutions(b, limit);
                    b.cells[idx] = 0;
                    if total >= limit {
                        break;
                    }
                }
            }
            total
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_hard_one() {
        let mut b = Board::parse(
            "8..........36......7..9.2...5...7.......457.....1...3...1....68..85...1..9....4..",
        )
        .unwrap();
        assert!(solve(&mut b));
        assert!(b.is_valid());
        assert!(b.cells.iter().all(|&c| c != 0));
    }

    #[test]
    fn empty_grid_is_not_unique() {
        let mut b = Board::empty();
        assert_eq!(count_solutions(&mut b, 2), 2);
    }
}
