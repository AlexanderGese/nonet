use crate::board::Board;

// plain text grid with the 3x3 box separators. the tui/web versions add color.
pub fn grid_lines(b: &Board) -> Vec<String> {
    let top = "┌───────┬───────┬───────┐";
    let mid = "├───────┼───────┼───────┤";
    let bot = "└───────┴───────┴───────┘";

    let mut out = vec![top.to_string()];
    for r in 0..9 {
        let mut line = String::from("│");
        for c in 0..9 {
            let v = b.get(r, c);
            line.push(' ');
            line.push(if v == 0 { '·' } else { (b'0' + v) as char });
            if c % 3 == 2 {
                line.push_str(" │");
            }
        }
        out.push(line);
        if r % 3 == 2 && r != 8 {
            out.push(mid.to_string());
        }
    }
    out.push(bot.to_string());
    out
}
