mod board;
mod render;
use board::Board;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| ".".repeat(81));
    match Board::parse(&arg) {
        Some(b) => {
            for l in render::grid_lines(&b) {
                println!("{l}");
            }
        }
        None => eprintln!("need 81 cells"),
    }
}
