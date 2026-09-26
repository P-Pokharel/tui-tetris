use std::io;
use std::time::{Instant, Duration};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders};
use ratatui::{DefaultTerminal, Frame};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_game(&mut terminal);
    ratatui::restore();
    result
}

fn run_game(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let delta_time = Duration::from_millis(16);
    let mut last_time = Instant::now();

    loop {
        terminal.draw(ui_draw)?;

        let timeout = delta_time.saturating_sub(last_time.elapsed());

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }

        if last_time.elapsed() >= delta_time {
            last_time = Instant::now();
        }
    }
}

fn ui_draw(frame: &mut Frame) {
    let area = frame.area();

    let vertical = Layout::vertical([
        Constraint::Percentage(5),
        Constraint::Percentage(90),
        Constraint::Percentage(5),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .split(vertical[1]);

    let content = horizontal[1];

    let main = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(2),
        Constraint::Length(16),
    ])
    .split(content);

    let board_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    frame.render_widget(board_block, main[0]);

    let side = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(1),
        Constraint::Length(4),
        Constraint::Fill(1),
    ])
    .split(main[2]);

    let next_block = Block::default()
        .borders(Borders::ALL)
        .title("NEXT")
        .border_style(Style::default().fg(Color::Blue));
    frame.render_widget(next_block, side[0]);

    let score_block = Block::default()
        .borders(Borders::ALL)
        .title("SCORE")
        .border_style(Style::default().fg(Color::Red));
    frame.render_widget(score_block, side[2]);
}
