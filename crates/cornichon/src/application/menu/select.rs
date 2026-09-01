//! [`SelectMenu`] — the simplest menu in the crate.
//!
//! A list of [`MenuItem`]s, a single cursor, arrow keys to navigate,
//! Enter to confirm, Esc to cancel. Optional `query` substring filter
//! to narrow the visible list. No metadata column, no header chrome.
//! This is the building block that the other menus extend.

use crate::application::menu::common::handle_common_key;
use crate::domain::{Error, MenuAction, MenuItem, Result, Selection, Theme};

/// A single-select list menu.
///
/// # Examples
///
/// ```rust
/// use cornichon::prelude::*;
///
/// let mut menu = SelectMenu::builder()
///     .items([
///         MenuItem::try_from(("Apple",  "A red fruit"))?,
///         MenuItem::try_from(("Banana", "A yellow fruit"))?,
///     ])
///     .build()?;
///
/// // Down arrow moves the cursor.
/// let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
/// assert_eq!(menu.selection(), 1);
///
/// # Ok::<(), cornichon::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct SelectMenu {
    items: Vec<MenuItem>,
    /// Indices into `items` that match the current `query()`. Recomputed
    /// on any query mutation. When `query()` is empty, this is
    /// `0..items.len()`.
    filtered: Vec<usize>,
    /// Current substring filter. Printable chars extend it; backspace
    /// removes the last character.
    query: String,
    selection: Selection,
    theme: Theme,
    title: Option<String>,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl SelectMenu {
    /// Start a builder.
    #[must_use]
    pub fn builder() -> SelectMenuBuilder {
        SelectMenuBuilder::default()
    }

    /// The items in display order.
    #[must_use]
    pub const fn items(&self) -> &[MenuItem] {
        self.items.as_slice()
    }

    /// The currently-selected index (0 when the list is empty).
    #[must_use]
    pub const fn selection(&self) -> usize {
        self.selection.index()
    }

    /// The active theme.
    #[must_use]
    pub const fn theme(&self) -> Theme {
        self.theme
    }

    /// Optional title rendered above the list.
    #[must_use]
    pub const fn title(&self) -> Option<&str> {
        // `Option::as_deref` is not yet stable as a const fn.
        match &self.title {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    /// Whether the adapter should draw a scrollbar indicator.
    #[must_use]
    pub const fn show_scrollbar(&self) -> bool {
        self.show_scrollbar
    }

    /// The current filter query.
    #[must_use]
    pub const fn query(&self) -> &str {
        self.query.as_str()
    }

    /// Indices into `items()` that match the current `query()`.
    ///
    /// When `query()` is empty, this is `0..items.len()`. As the user
    /// types into the query, the visible list narrows.
    #[must_use]
    pub const fn filtered_indices(&self) -> &[usize] {
        self.filtered.as_slice()
    }

    /// Apply a key event. Returns the resulting [`MenuAction`].
    pub fn update(&mut self, event: crate::ports::KeyEvent) -> MenuAction {
        match event.code {
            crate::ports::KeyCode::Char(c)
                if !event
                    .modifiers
                    .contains(crate::ports::KeyModifiers::CONTROL) =>
            {
                // Hotkey jump takes priority over filter typing. If
                // any item has `key == Some(c)`, jump to it; otherwise
                // append the char to the filter query. This preserves
                // the original `SelectMenu` hotkey behavior (item 5 of
                // the 15-item plan) while adding the new filter (item 7).
                if let Some(idx) = self.items.iter().position(|i| i.key == Some(c)) {
                    self.query.clear();
                    // `recompute_filtered` resets `self.selection` to
                    // row 0 (since the query is now empty and all items
                    // are visible). Set the cursor to the hotkey match
                    // AFTER the reset, so it isn't clobbered.
                    self.recompute_filtered();
                    self.selection.set(idx);
                } else {
                    self.query.push(c);
                    self.recompute_filtered();
                }
                MenuAction::None
            }
            crate::ports::KeyCode::Backspace => {
                self.query.pop();
                self.recompute_filtered();
                MenuAction::None
            }
            _ => {
                // Build a filtered view for navigation so the selection's
                // count and item lookup track the visible list. Allocates
                // a small Vec per non-query event, but typical menus are
                // short and this avoids the indexing-slicing lint.
                let visible: Vec<MenuItem> = self
                    .filtered
                    .iter()
                    .filter_map(|&i| self.items.get(i).cloned())
                    .collect();
                match handle_common_key(&mut self.selection, &visible, event) {
                    MenuAction::Selected(item) => MenuAction::Selected(item),
                    other => other,
                }
            }
        }
    }

    /// Recompute the `filtered` index list from the current `query`.
    /// Resets the selection to row 0 so the cursor lands on the first
    /// match.
    fn recompute_filtered(&mut self) {
        self.filtered = crate::application::filter::filter_indices(&self.items, &self.query);
        self.selection = Selection::new(self.filtered.len());
    }

    /// Dispatch a unified [`crate::ports::MenuEvent`] (keyboard OR mouse)
    /// to this menu. Default `on_mouse` returns [`MenuAction::None`].
    pub fn dispatch(&mut self, event: impl Into<crate::ports::MenuEvent>) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl crate::ports::MenuDispatch for SelectMenu {
    fn key_input(&mut self, event: crate::ports::KeyEvent) -> MenuAction {
        self.update(event)
    }

    fn on_mouse(&mut self, event: crate::ports::MouseEvent) -> MenuAction {
        use crate::ports::MouseEventKind;
        match event.kind {
            MouseEventKind::Down(crate::ports::MouseButton::Left) => {
                let row = event.row as usize;
                if row < self.items.len() {
                    self.selection.set(row);
                }
                MenuAction::None
            }
            _ => MenuAction::None,
        }
    }
}

/// Builder for [`SelectMenu`].
///
/// Constructed via [`SelectMenu::builder()`]; the defaults are an empty item
/// list, the [`Theme::default`] (Claude) palette, and no title. Call
/// [`SelectMenuBuilder::build`] to produce the menu — it returns an
/// [`Error::EmptyMenu`] if you forget to add any items.
#[derive(Debug, Clone, Default)]
pub struct SelectMenuBuilder {
    items: Vec<MenuItem>,
    query: String,
    theme: Theme,
    title: Option<String>,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl SelectMenuBuilder {
    /// Set the items (replaces any previously set).
    #[must_use]
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = MenuItem>,
    {
        self.items = items.into_iter().collect();
        self
    }

    /// Add a single item.
    #[must_use]
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set the initial filter query.
    ///
    /// Equivalent to the user having typed these characters at runtime.
    /// Mostly useful for tests and "remember the user's last query"
    /// style persistence.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = value.into();
        self
    }

    /// Set the theme.
    #[must_use]
    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set an optional title rendered above the list.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set whether the adapter should draw a scrollbar indicator.
    #[must_use]
    pub const fn show_scrollbar(mut self, on: bool) -> Self {
        self.show_scrollbar = on;
        self
    }

    /// Build the menu. Errors if no items were added.
    pub fn build(self) -> Result<SelectMenu> {
        if self.items.is_empty() {
            return Err(Error::EmptyMenu);
        }
        let count = self.items.len();
        let mut menu = SelectMenu {
            items: self.items,
            filtered: (0..count).collect(),
            query: self.query,
            selection: Selection::new(count),
            theme: self.theme,
            title: self.title,
            show_scrollbar: self.show_scrollbar,
        };
        if !menu.query.is_empty() {
            menu.recompute_filtered();
        }
        Ok(menu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{KeyCode, KeyEvent, KeyModifiers};

    fn ev(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn builder_rejects_empty() {
        let result = SelectMenu::builder().build();
        assert!(matches!(result, Err(Error::EmptyMenu)));
    }

    #[test]
    fn down_advances_cursor() -> Result<()> {
        let mut menu = SelectMenu::builder()
            .items([
                MenuItem::try_from(("a", "first"))?,
                MenuItem::try_from(("b", "second"))?,
                MenuItem::try_from(("c", "third"))?,
            ])
            .build()?;
        assert_eq!(menu.selection(), 0);
        let _ = menu.update(ev(KeyCode::Down));
        assert_eq!(menu.selection(), 1);
        Ok(())
    }

    #[test]
    fn down_wraps_at_bottom() -> Result<()> {
        let mut menu = SelectMenu::builder()
            .items([
                MenuItem::try_from(("a", "first"))?,
                MenuItem::try_from(("b", "second"))?,
            ])
            .build()?;
        let _ = menu.update(ev(KeyCode::Down));
        let _ = menu.update(ev(KeyCode::Down));
        assert_eq!(menu.selection(), 0);
        Ok(())
    }

    #[test]
    fn enter_returns_selected_item() -> Result<()> {
        let mut menu = SelectMenu::builder()
            .items([
                MenuItem::try_from(("a", "first"))?,
                MenuItem::try_from(("b", "second"))?,
            ])
            .build()?;
        let _ = menu.update(ev(KeyCode::Down));
        let action = menu.update(ev(KeyCode::Enter));
        assert!(matches!(action, MenuAction::Selected(ref i) if i.name == "b"));
        Ok(())
    }

    #[test]
    fn esc_returns_cancelled() -> Result<()> {
        let mut menu = SelectMenu::builder()
            .items([MenuItem::try_from(("a", "first"))?])
            .build()?;
        let action = menu.update(ev(KeyCode::Esc));
        assert_eq!(action, MenuAction::Cancelled);
        Ok(())
    }

    #[test]
    fn hotkey_jumps_to_matching_item() -> Result<()> {
        let mut menu = SelectMenu::builder()
            .items([
                MenuItem::try_from(("yes", "approve"))?.with_key('y'),
                MenuItem::try_from(("no", "reject"))?.with_key('n'),
            ])
            .build()?;
        let _ = menu.update(ev(KeyCode::Char('n')));
        assert_eq!(menu.selection(), 1);
        Ok(())
    }
}
