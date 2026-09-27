use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Block;

use crate::tetromino::Shape;

pub const CELL_W: u16 = 2;
const CELL_H: u16 = 1;


pub fn draw_cell(frame: &mut Frame, area: Rect, x: i32, y: i32, color: Color) {
    if x < 0 || y < 0 {
        return;
    }
    let cell = Rect {
        x: area.x + x as u16 * CELL_W,
        y: area.y + y as u16 * CELL_H,
        width: CELL_W,
        height: CELL_H,
    };
    if cell.right() > area.right() || cell.bottom() > area.bottom() {
        return;
    }
    frame.render_widget(
        Block::new().style(Style::default().bg(color)),
        cell
    );
}

pub fn draw_shape(
    frame: &mut Frame,
    area: Rect,
    cells: &Shape,
    color: Color,
    px: i32,
    py: i32
) {
    for (r, rows) in cells.iter().enumerate() {
        for (c, cell) in rows.iter().enumerate() {
            if *cell == 0 { continue; }
            draw_cell(frame, area, c as i32 + px, r as i32 + py, color);
        }
    }
}
