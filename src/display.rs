use ratatui::layout::{Constraint, Rect};
use ratatui::{buffer::Buffer, layout::Layout, widgets::{Block, Paragraph, Widget}};
use crate::game::GameState;

impl Widget for GameState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..4).map(|_| Constraint::Length(5));
        let row_constraints = (0..4).map(|_| Constraint::Length(3));

        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("{}", self.board[i/4][i%4]))
                .block(Block::bordered())
                .centered()
                .render(cell, buf);
        }
    }
}

