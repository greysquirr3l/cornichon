//! `Widget` impl for `&Spinner` — renders the current frame glyph and
//! (optionally) a dim label to its right.

use crate::application::Spinner;
use crate::ports::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

impl Widget for &Spinner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let theme = self.theme();
        let frame = self.current_frame();
        let label = self.label();
        let mut spans = Vec::with_capacity(3);
        spans.push(Span::styled(frame, Style::default().fg(theme.fg)));
        if let Some(text) = label {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(text, Style::default().fg(theme.fg_dim)));
        }
        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
