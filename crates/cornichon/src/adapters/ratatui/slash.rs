//! `Widget` impl for `&SlashMenu` — the standard ratatui render entrypoint.

use crate::application::SlashMenu;
use crate::ports::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use super::common::render_row;

impl Widget for &SlashMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }
        let theme = self.theme();
        let max_y = area.y.saturating_add(area.height);

        // Items area: leaves the last row for the query input.
        let items_height = area.height.saturating_sub(1);
        let items = self.filtered_indices();
        let original_items = self.items();
        let selected = self.selection();
        for (i, &orig_idx) in items.iter().enumerate() {
            let y = area.y.saturating_add(u16::try_from(i).unwrap_or(u16::MAX));
            if y >= area.y.saturating_add(items_height) {
                break;
            }
            let Some(item) = original_items.get(orig_idx) else {
                continue;
            };
            let row = Rect {
                x: area.x,
                y,
                width: area.width,
                height: 1,
            };
            render_row(item, i == selected, theme, row, buf);
        }

        // Query input row.
        let query_y = max_y.saturating_sub(1);
        let prefix = "/";
        let query = self.query();
        let line = Line::from(vec![
            Span::styled(prefix, Style::default().fg(theme.fg)),
            Span::styled(query, Style::default().fg(theme.fg)),
        ]);
        buf.set_line(area.x, query_y, &line, area.width);
    }
}
