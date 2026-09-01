//! `Widget` impl for `&ProjectPicker` — the standard ratatui render entrypoint.

use crate::application::ProjectPicker;
use crate::domain::item::AnnotatedItem;
use crate::ports::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

impl Widget for &ProjectPicker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }
        let theme = self.theme();
        let max_y = area.y.saturating_add(area.height);
        let mut y = area.y;

        // Title (in accent).
        let title_line = Line::from(Span::styled(
            self.title(),
            Style::default().fg(theme.accent),
        ));
        buf.set_line(area.x, y, &title_line, area.width);
        y = y.saturating_add(1);

        // Description (in fg_dim).
        if y < max_y {
            let desc_line = Line::from(Span::styled(
                self.description(),
                Style::default().fg(theme.fg_dim),
            ));
            buf.set_line(area.x, y, &desc_line, area.width);
            y = y.saturating_add(1);
        }

        // Items: each project gets its number prefix + name + annotation.
        let items = self.projects();
        let custom_idx = items.len();
        let selected = self.selection();
        for (i, project) in items.iter().enumerate() {
            if y >= max_y {
                break;
            }
            render_project_row(
                project,
                i,
                i == selected,
                theme,
                Rect {
                    x: area.x,
                    y,
                    width: area.width,
                    height: 1,
                },
                buf,
            );
            y = y.saturating_add(1);
        }

        // Trailing "custom" option.
        if y < max_y {
            let is_custom = selected == custom_idx;
            let fg = if is_custom { theme.fg } else { theme.fg_muted };
            let label = self.custom_label();
            let line = Line::from(vec![
                Span::styled("z ", Style::default().fg(fg)),
                Span::styled(
                    if is_custom { "(●) " } else { "(○) " },
                    Style::default().fg(fg),
                ),
                Span::styled(label, Style::default().fg(fg)),
            ]);
            buf.set_line(area.x, y, &line, area.width);
        }
    }
}

fn render_project_row(
    project: &AnnotatedItem,
    index: usize,
    is_selected: bool,
    theme: crate::domain::Theme,
    area: Rect,
    buf: &mut Buffer,
) {
    let fg = if is_selected {
        theme.fg
    } else {
        theme.fg_muted
    };
    let prefix = format!("{} ", index + 1);
    let marker = if is_selected { "(●) " } else { "(○) " };
    let annotation = project.annotation.clone().unwrap_or_default();
    let line = Line::from(vec![
        Span::styled(prefix, Style::default().fg(fg)),
        Span::styled(marker, Style::default().fg(fg)),
        Span::styled(project.name.clone(), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(
            project.description.clone(),
            Style::default().fg(theme.fg_muted),
        ),
        Span::raw("  "),
        Span::styled(annotation, Style::default().fg(theme.fg_muted)),
    ]);
    buf.set_line(area.x, area.y, &line, area.width);
}
