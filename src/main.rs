mod board;
mod generator;
mod render;
mod rng;
mod solver;

use board::Board;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "nonet", version, about = "A Sudoku solver + generator with a visual CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Solve an 81-char puzzle ('.' or 0 = empty)
    Solve { puzzle: String },
    /// Generate a fresh puzzle (easy | medium | hard | expert)
    Generate {
        #[arg(short, long, default_value = "medium")]
        difficulty: String,
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Generate a puzzle and solve it
    Demo,
}

fn seed_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x1234_5678)
}

fn show(title: &str, b: &Board) {
    println!("\n  \x1b[1;35m{title}\x1b[0m");
    for l in render::grid_lines(b) {
        println!("  \x1b[90m{l}\x1b[0m");
    }
}

fn main() -> ExitCode {
    match Cli::parse().cmd {
        Cmd::Solve { puzzle } => {
            let Some(mut b) = Board::parse(&puzzle) else {
                eprintln!("\x1b[31merror:\x1b[0m need 81 cells (digits, '.' or '0')");
                return ExitCode::FAILURE;
            };
            show("puzzle", &b);
            if solver::solve(&mut b) {
                show("solution", &b);
            } else {
                eprintln!("\x1b[31mno solution\x1b[0m");
                return ExitCode::FAILURE;
            }
        }
        Cmd::Generate { difficulty, seed } => {
            let mut r = rng::Rng::new(seed.unwrap_or_else(seed_now));
            let p = generator::generate(&mut r, generator::clues_for(&difficulty));
            show(&format!("{difficulty} · {} clues", p.board.clue_count()), &p.board);
            println!("  \x1b[90mline:\x1b[0m {}", p.board.to_line());
        }
        Cmd::Demo => {
            let mut r = rng::Rng::new(seed_now());
            let p = generator::generate(&mut r, 32);
            show(&format!("generated · {} clues", p.board.clue_count()), &p.board);
            let mut s = p.board;
            solver::solve(&mut s);
            show("solved", &s);
        }
    }
    ExitCode::SUCCESS
}
