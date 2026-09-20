use std::io;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, style::Stylize, widgets::Paragraph};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_game(&mut terminal);
    ratatui::restore();
    result
}

fn run_game(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui_draw(frame))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui_draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello, Tetris")
        .centered().bold().red();
    frame.render_widget(greeting, frame.area());
}
