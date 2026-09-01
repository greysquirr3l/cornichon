//! `Widget for &Type` impls for the form widgets: `TextInput`, `Checkbox`,
//! `RadioGroup`.
//!
//! Renders via the standard ratatui `Widget` trait; consumers use
//! `frame.render_widget(&text_input, area)` like any other widget.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::domain::forms::{Checkbox, RadioGroup, TextInput};

// ---------------------------------------------------------------------------
// TextInput
// ---------------------------------------------------------------------------

impl Widget for &TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        // Compute the visible slice. For long values, we keep the cursor
        // visible by trimming from the left.
        let value = &self.value;
        let cursor = self.cursor;
        let max_w = area.width as usize;
        let (start, _) = if value.len() > max_w && cursor >= max_w {
            (cursor - max_w + 1, cursor)
        } else {
            (0, value.len().min(max_w))
        };
        let end = (start + max_w).min(value.len());
        let visible = &value[start..end];
        let cursor_in_visible = cursor.saturating_sub(start);
        let style = Style::default().fg(Color::Reset);
        let placeholder_style = Style::default().fg(Color::DarkGray);
        // Render the value (or placeholder if empty) and the cursor.
        let mut spans: Vec<Span> = Vec::with_capacity(visible.len() + 1);
        if value.is_empty() {
            if let Some(p) = self.placeholder.as_deref() {
                spans.push(Span::styled(p, placeholder_style));
            }
            spans.push(Span::styled(" ", style));
        } else {
            for c in visible.chars() {
                spans.push(Span::styled(c.to_string(), style));
            }
        }
        // Cursor marker — a reversed block over the current cell.
        if value.is_empty() && self.placeholder.is_none() {
            spans.push(Span::styled(" ", style));
        } else if !value.is_empty() {
            // We can't easily mutate the spans vec; just append a cursor
            // marker after the visible chars. For an empty input the
            // marker is the only thing rendered.
            spans.push(Span::styled(" ", style));
        }
        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
        // Draw the cursor cell with reversed colors. The cursor is bounded
        // by the visible slice, so truncation is impossible in practice;
        // `try_from` + `unwrap_or(u16::MAX)` is the project's standard
        // pattern for safe `usize` → `u16` conversion.
        let cursor_x = if value.is_empty() {
            0
        } else {
            u16::try_from(cursor_in_visible).unwrap_or(u16::MAX)
        };
        if area.width > cursor_x {
            let cell = &mut buf[(area.x + cursor_x, area.y)];
            cell.set_style(Style::default().bg(Color::White).fg(Color::Black));
        }
    }
}

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

impl Widget for &Checkbox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        let mark = if self.checked { "[X]" } else { "[ ]" };
        let style = if self.checked {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let line = Line::from(vec![
            Span::styled(mark, style),
            Span::raw(" "),
            Span::raw(self.label.clone()),
        ]);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

impl Widget for &RadioGroup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.options.is_empty() || area.width == 0 || area.height == 0 {
            return;
        }
        for (i, label) in self.options.iter().enumerate() {
            let y = area
                .y
                .saturating_add(u16::try_from(i).unwrap_or(u16::MAX));
            if y >= area.y + area.height {
                break;
            }
            let is_selected = i == self.selected;
            let mark = if is_selected { "(•)" } else { "( )" };
            let style = if is_selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let line = Line::from(vec![
                Span::styled(mark, style),
                Span::raw(" "),
                Span::raw(label.clone()),
            ]);
            buf.set_line(area.x, y, &line, area.width);
        }
    }
}
