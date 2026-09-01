//! `Widget` impl for `&ShortcutsModal` — the standard ratatui render entrypoint.

use crate::application::ShortcutsModal;
use crate::ports::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

impl Widget for &ShortcutsModal {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }
        let theme = self.theme();
        let max_y = area.y.saturating_add(area.height);
        let mut y = area.y;
        let selected = self.selection();
        let expanded = self.expanded();
        for (i, group) in self.groups().iter().enumerate() {
            if y >= max_y {
                break;
            }
            let is_selected = i == selected;
            let is_expanded = expanded == Some(i);
            let header_color = if is_selected { theme.fg } else { theme.fg_muted };

            if is_expanded {
                let header = Line::from(vec![
                    Span::styled("◆ ", Style::default().fg(theme.accent)),
                    Span::styled(group.label.clone(), Style::default().fg(header_color)),
                ]);
                buf.set_line(area.x, y, &header, area.width);
                y = y.saturating_add(1);

                for item in &group.items {
                    if y >= max_y {
                        break;
                    }
                    let line = Line::from(vec![
                        Span::raw("    "),
                        Span::styled(item.action.clone(), Style::default().fg(theme.fg)),
                        Span::raw("  "),
                        Span::styled(item.keys.clone(), Style::default().fg(theme.fg_dim)),
                    ]);
                    buf.set_line(area.x, y, &line, area.width);
                    y = y.saturating_add(1);
                }
            } else {
                let prefix = if is_selected { "› " } else { "  " };
                let line = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(header_color)),
                    Span::styled(group.label.clone(), Style::default().fg(header_color)),
                    Span::styled(format!(" ({})", group.collapsed_count()), Style::default().fg(theme.fg_muted)),
                ]);
                buf.set_line(area.x, y, &line, area.width);
                y = y.saturating_add(1);
            }
        }
    }
}
