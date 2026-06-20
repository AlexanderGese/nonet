use crate::board::Board;
use crate::generator;
use crate::rng::Rng;
use crate::solver::{self, Step};
use crossterm::{event, execute, terminal};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use std::io;
use std::time::{Duration, Instant};

struct App {
    given: [bool; 81],
    start: Board, // puzzle with only the clues
    board: Board, // what's on screen right now
    steps: Vec<Step>,
    pos: usize,
    playing: bool,
    delay_ms: u64,
    placed: usize,
    backtracks: usize,
    last: Option<usize>,
    diff: String,
    seed: u64,
}

fn build(seed: u64, diff: &str) -> App {
    let mut r = Rng::new(seed);
    let p = generator::generate(&mut r, generator::clues_for(diff));
    let given = std::array::from_fn(|i| p.board.cells[i] != 0);

    let mut trace = p.board;
    let mut steps = Vec::new();
    solver::solve_trace(&mut trace, &mut steps);

    App {
        given,
        start: p.board,
        board: p.board,
        steps,
        pos: 0,
        playing: true,
        delay_ms: 35,
        placed: 0,
        backtracks: 0,
        last: None,
        diff: diff.to_string(),
        seed,
    }
}

impl App {
    fn step(&mut self) {
        if self.pos >= self.steps.len() {
            self.playing = false;
            return;
        }
        match self.steps[self.pos] {
            Step::Place(i, v) => {
                self.board.cells[i] = v;
                self.placed += 1;
                self.last = Some(i);
            }
            Step::Erase(i) => {
                self.board.cells[i] = 0;
                self.backtracks += 1;
                self.last = Some(i);
            }
        }
        self.pos += 1;
    }

    fn reset(&mut self) {
        self.board = self.start;
        self.pos = 0;
        self.placed = 0;
        self.backtracks = 0;
        self.last = None;
        self.playing = true;
    }

    fn done(&self) -> bool {
        self.pos >= self.steps.len()
    }
}

const TOP: &str = "┌───────┬───────┬───────┐";
const MID: &str = "├───────┼───────┼───────┤";
const BOT: &str = "└───────┴───────┴───────┘";

fn grid(app: &App) -> Vec<Line<'static>> {
    let sep = Style::new().fg(Color::DarkGray);
    let mut out = vec![Line::styled(TOP, sep)];
    for r in 0..9 {
        let mut spans = vec![Span::styled("│", sep)];
        for c in 0..9 {
            let i = r * 9 + c;
            let v = app.board.cells[i];
            let ch = if v == 0 { '·' } else { (b'0' + v) as char };
            let st = if Some(i) == app.last {
                let bg = if v == 0 { Color::Red } else { Color::Yellow };
                Style::new().fg(Color::Black).bg(bg)
            } else if v == 0 {
                Style::new().fg(Color::DarkGray)
            } else if app.given[i] {
                Style::new().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::Green)
            };
            spans.push(Span::styled(format!(" {ch}"), st));
            if c % 3 == 2 {
                spans.push(Span::styled(" │", sep));
            }
        }
        out.push(Line::from(spans));
        if r % 3 == 2 && r != 8 {
            out.push(Line::styled(MID, sep));
        }
    }
    out.push(Line::styled(BOT, sep));
    out
}

fn view(app: &App) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let head = format!(
        "{} · {} clues · placed {} · backtracks {}",
        app.diff,
        app.start.clue_count(),
        app.placed,
        app.backtracks
    );
    lines.push(Line::styled(head, Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD)));
    lines.push(Line::raw(""));
    lines.extend(grid(app));
    lines.push(Line::raw(""));
    if app.done() {
        lines.push(Line::styled("solved ✓", Style::new().fg(Color::Green).add_modifier(Modifier::BOLD)));
    } else if app.playing {
        lines.push(Line::styled(format!("solving…  step {}/{}", app.pos, app.steps.len()), Style::new().fg(Color::Yellow)));
    } else {
        lines.push(Line::styled("paused", Style::new().fg(Color::DarkGray)));
    }
    lines.push(Line::styled(
        "space play/pause · n new · r reset · s solve · +/- speed · 1-4 difficulty · q quit",
        Style::new().fg(Color::DarkGray),
    ));
    lines
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height.saturating_sub(h)) / 2,
        width: w,
        height: h,
    }
}

pub fn run(seed: u64, diff: &str) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = build(seed, diff);
    let mut tick = Instant::now();
    let res = loop {
        if let Err(e) = term.draw(|f| {
            let block = Block::bordered()
                .title(" nonet — sudoku solver ")
                .border_style(Style::new().fg(Color::DarkGray));
            let p = Paragraph::new(view(&app)).block(block).alignment(Alignment::Center);
            f.render_widget(p, centered(f.area(), 50, 22));
        }) {
            break Err(e);
        }

        let wait = if app.playing { app.delay_ms.max(1) } else { 120 };
        match event::poll(Duration::from_millis(wait)) {
            Ok(true) => {
                if let Ok(event::Event::Key(k)) = event::read() {
                    if k.kind == event::KeyEventKind::Press {
                        use event::KeyCode::*;
                        match k.code {
                            Char('q') | Esc => break Ok(()),
                            Char(' ') => app.playing = !app.playing,
                            Char('n') => app = build(app.seed.wrapping_add(0x9e37), &app.diff),
                            Char('r') => app.reset(),
                            Char('s') => while !app.done() { app.step() },
                            Char('+') | Char('=') => app.delay_ms = app.delay_ms.saturating_sub(8).max(1),
                            Char('-') | Char('_') => app.delay_ms = (app.delay_ms + 8).min(400),
                            Char('1') => app = build(app.seed, "easy"),
                            Char('2') => app = build(app.seed, "medium"),
                            Char('3') => app = build(app.seed, "hard"),
                            Char('4') => app = build(app.seed, "expert"),
                            _ => {}
                        }
                    }
                }
            }
            Ok(false) => {} // timed out, fall through to the tick
            Err(e) => break Err(e),
        }

        if app.playing && tick.elapsed() >= Duration::from_millis(app.delay_ms) {
            app.step();
            tick = Instant::now();
        }
    };

    terminal::disable_raw_mode()?;
    execute!(term.backend_mut(), terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;
    res
}
