//! `WidgetRef` impl for [`SelectMenu`](crate::application::SelectMenu).

use crate::application::SelectMenu;
use crate::ports::{Buffer, Rect, Widget};

use super::common::{render_row, render_title};

impl Widget for &SelectMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = render_title(self.title(), self.theme(), area, buf);
        let selected = self.selection();
        for (display_idx, &orig_idx) in self.filtered_indices().iter().enumerate() {
            let Some(item) = self.items().get(orig_idx) else {
                break;
            };
            // Saturating y so we never wrap past the end of the buffer.
            let y = area
                .y
                .saturating_add(u16::try_from(display_idx).unwrap_or(u16::MAX));
            if y >= area.y.saturating_add(area.height) {
                break;
            }
            let row = Rect {
                x: area.x,
                y,
                width: area.width,
                height: 1,
            };
            render_row(item, display_idx == selected, self.theme(), row, buf);
        }
    }
}
