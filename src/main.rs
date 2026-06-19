mod board;
mod render;
mod rng;
mod solver;
use board::Board;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| ".".repeat(81));
    let Some(mut b) = Board::parse(&arg) else {
        eprintln!("need 81 cells");
        return;
    };
    for l in render::grid_lines(&b) {
        println!("{l}");
    }
    if solver::solve(&mut b) {
        println!();
        for l in render::grid_lines(&b) {
            println!("{l}");
        }
    } else {
        eprintln!("no solution");
    }
}
