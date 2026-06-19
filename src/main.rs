mod board;
mod generator;
mod render;
mod rng;
mod solver;

fn main() {
    // generate one and solve it
    let mut r = rng::Rng::new(12345);
    let p = generator::generate(&mut r, 32);
    for l in render::grid_lines(&p.board) {
        println!("{l}");
    }
    let mut s = p.board;
    solver::solve(&mut s);
    println!();
    for l in render::grid_lines(&s) {
        println!("{l}");
    }
}
