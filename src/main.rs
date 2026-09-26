use std::io;
use std::time::{Instant, Duration};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders};
use ratatui::{DefaultTerminal, Frame};

mod tetromino;

const CELL_W: u16 = 2;
const CELL_H: u16 = 1;
const BOARD_H: usize = 20;
const BOARD_W: usize = 10;
const ACTUAL_BOARD_H: u16 = BOARD_H as u16 + 2;
const ACTUAL_BOARD_W: u16 = BOARD_W as u16 * CELL_W + 2;

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

    let [vertical] = Layout::vertical([
        Constraint::Length(ACTUAL_BOARD_H)
    ]).flex(Flex::Center).areas(area);

    let [board_area, _, side_area] = Layout::horizontal([
        Constraint::Length(ACTUAL_BOARD_W),
        Constraint::Length(2),
        Constraint::Length(16)
    ]).flex(Flex::Center).areas(vertical);

    let board_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    frame.render_widget(board_block, board_area);

    let [next_area, _, score_area] = Layout::vertical([
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Length(4),
    ])
    .areas(side_area);

    let next_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    frame.render_widget(next_block, next_area);

    let score_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    frame.render_widget(score_block, score_area);
}
