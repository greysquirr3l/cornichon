//! `Widget` impl for `&PermissionPrompt` — the standard ratatui render entrypoint.

use crate::application::PermissionPrompt;
use crate::ports::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use super::common::render_row;

impl Widget for &PermissionPrompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }
        let theme = self.theme();

        // Row 0: title (in accent color).
        let title_line = Line::from(Span::styled(self.title(), Style::default().fg(theme.accent)));
        buf.set_line(area.x, area.y, &title_line, area.width);
        let mut y = area.y.saturating_add(1);

        // Row 1: command (in fg_dim).
        if y < area.y.saturating_add(area.height) {
            let cmd_line = Line::from(Span::styled(
                self.command(),
                Style::default().fg(theme.fg_dim),
            ));
            buf.set_line(area.x, y, &cmd_line, area.width);
            y = y.saturating_add(1);
        }

        // Row 2: question.
        if y < area.y.saturating_add(area.height) {
            let q_line = Line::from(Span::styled(
                self.question(),
                Style::default().fg(theme.fg),
            ));
            buf.set_line(area.x, y, &q_line, area.width);
            y = y.saturating_add(1);
        }

        // Remaining rows: items.
        let items_area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: area.y.saturating_add(area.height).saturating_sub(y),
        };
        let items = self.items();
        let selected = self.selection();
        for (i, item) in items.iter().enumerate() {
            let row_y = items_area.y.saturating_add(u16::try_from(i).unwrap_or(u16::MAX));
            if row_y >= items_area.y.saturating_add(items_area.height) {
                break;
            }
            let row = Rect { x: items_area.x, y: row_y, width: items_area.width, height: 1 };
            render_row(item, i == selected, theme, row, buf);
        }
    }
}
