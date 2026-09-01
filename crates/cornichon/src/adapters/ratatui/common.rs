//! Shared rendering helpers.
//!
//! Row rendering is factored out so each menu's `Widget` impl is a short
//! top-level loop over its items.

use crate::domain::{MenuItem, Theme};
use crate::ports::{Buffer, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Render a single row (marker + name + description) into `buf` at the
/// given `area`. Used by every menu's `Widget` impl.
///
/// `is_selected` controls the marker glyph and foreground/background colors.
pub(crate) fn render_row(
    item: &MenuItem,
    is_selected: bool,
    theme: Theme,
    area: Rect,
    buf: &mut Buffer,
) {
    let marker_text = if is_selected {
        theme.marker
    } else {
        // Unselected rows leave `marker_width` cells of blank space so the
        // text column stays aligned across rows.
        match theme.marker_width {
            0 => "",
            1 => " ",
            2 => "  ",
            _ => "   ",
        }
    };
    let fg = if is_selected {
        theme.fg
    } else {
        theme.fg_muted
    };
    let bg = if is_selected {
        theme.selected_bg.unwrap_or(Color::Reset)
    } else {
        Color::Reset
    };

    let marker_style = Style::default().fg(fg).bg(bg);
    let name_style = Style::default().fg(fg).bg(bg);
    let desc_style = Style::default().fg(theme.fg_dim);

    let line = Line::from(vec![
        Span::styled(marker_text, marker_style),
        Span::raw(" "),
        Span::styled(item.name.as_str(), name_style),
        Span::raw("  "),
        Span::styled(item.description.as_str(), desc_style),
    ]);

    buf.set_line(area.x, area.y, &line, area.width);
}

/// Render an optional title row at the top of `area`, returning the
/// remaining area for items.
pub(crate) fn render_title(
    title: Option<&str>,
    theme: Theme,
    area: Rect,
    buf: &mut Buffer,
) -> Rect {
    match title {
        Some(t) if area.height > 0 => {
            let line = Line::from(Span::styled(t, Style::default().fg(theme.fg)));
            buf.set_line(area.x, area.y, &line, area.width);
            Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: area.height.saturating_sub(1),
            }
        }
        _ => area,
    }
}
