mod board;
use board::Board;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| ".".repeat(81));
    match Board::parse(&arg) {
        Some(b) => println!("{}", b.to_line()),
        None => eprintln!("need 81 cells"),
    }
}
