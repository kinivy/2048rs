use ratatui::layout::{Constraint, Rect};
use ratatui::{buffer::Buffer, layout::Layout, widgets::{Block, Paragraph, Widget}};
use ratatui::style::{Color, Style};
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
            let value = self.board[i/4][i%4];
            Paragraph::new(get_display_value(&value).to_string())
                .block(Block::bordered())
                .centered()
                .style(
                    Style::new()
                    .bg(get_color(&value)),
                )
                .render(cell, buf);
        }
    }
}

fn get_display_value(value:&u32) -> String {
   if *value == 0 {
       "".to_string()
   } else {
       value.to_string()
   }
}


fn get_color(value:&u32) -> Color {
    match *value {
        2 => Color::Yellow,
        4 => Color::Red,
        8 => Color::Blue,
        16 => Color::Magenta,
        32 => Color::Cyan,
        _ => Color::Reset
    }
}
