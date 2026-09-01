//! [`SlashMenu`] — the filterable `/command` palette.

use crate::domain::{Error, MenuAction, MenuItem, Result, Selection, Theme};

/// The default commands — matches the brainless capture.
pub const DEFAULT_COMMANDS: &[(&str, &str)] = &[
    ("/agents", "Manage subagents for specialized tasks"),
    ("/clear", "Clear conversation history and free up context"),
    ("/compact", "Summarize the conversation to save context"),
    ("/init", "Initialize a CLAUDE.md with codebase docs"),
    ("/model", "Change the model for this session"),
    ("/review", "Review a pull request"),
];

/// Parse the default-command table into typed [`MenuItem`]s.
pub fn default_commands() -> Result<Vec<MenuItem>> {
    DEFAULT_COMMANDS
        .iter()
        .map(|(name, desc)| MenuItem::try_from((String::from(*name), String::from(*desc))))
        .collect()
}

/// A filterable `/command` palette.
///
/// Typing into the query narrows the visible list. Arrow keys move within
/// the filtered view; Enter selects; Esc cancels; Backspace pops the query.
#[derive(Debug, Clone)]
pub struct SlashMenu {
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
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl SlashMenu {
    /// Start a builder.
    #[must_use]
    pub fn builder() -> SlashMenuBuilder {
        SlashMenuBuilder::default()
    }

    /// Convenience: builder with the Claude default commands populated.
    ///
    /// Equivalent to `SlashMenu::builder().with_defaults()?.build()?`.
    pub fn with_defaults() -> Result<Self> {
        Self::builder().with_defaults()?.build()
    }

    /// The original (unfiltered) items.
    #[must_use]
    pub const fn items(&self) -> &[MenuItem] {
        self.items.as_slice()
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

    /// The cursor position within `filtered_indices()`.
    #[must_use]
    pub const fn selection(&self) -> usize {
        self.selection.index()
    }

    /// The active theme.
    #[must_use]
    pub const fn theme(&self) -> Theme {
        self.theme
    }

    /// Whether the adapter should draw a scrollbar indicator.
    #[must_use]
    pub const fn show_scrollbar(&self) -> bool {
        self.show_scrollbar
    }

    /// Apply a key event. Returns the resulting [`MenuAction`].
    pub fn update(&mut self, event: crate::ports::KeyEvent) -> MenuAction {
        match event.code {
            crate::ports::KeyCode::Up => {
                self.selection.up();
                MenuAction::None
            }
            crate::ports::KeyCode::Down => {
                self.selection.down();
                MenuAction::None
            }
            crate::ports::KeyCode::Enter => self
                .filtered
                .get(self.selection.index())
                .and_then(|&i| self.items.get(i))
                .map_or(MenuAction::None, |item| MenuAction::Selected(item.clone())),
            crate::ports::KeyCode::Esc => MenuAction::Cancelled,
            crate::ports::KeyCode::Backspace => {
                self.query.pop();
                self.recompute_filtered();
                MenuAction::None
            }
            crate::ports::KeyCode::Char(c)
                if !event
                    .modifiers
                    .contains(crate::ports::KeyModifiers::CONTROL) =>
            {
                self.query.push(c);
                self.recompute_filtered();
                MenuAction::None
            }
            _ => MenuAction::None,
        }
    }

    /// Recompute the `filtered` index list from the current `query()`.
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

impl crate::ports::MenuDispatch for SlashMenu {
    fn key_input(&mut self, event: crate::ports::KeyEvent) -> MenuAction {
        self.update(event)
    }

    fn on_mouse(&mut self, event: crate::ports::MouseEvent) -> MenuAction {
        let _ = event;
        MenuAction::None
    }
}

/// Builder for [`SlashMenu`].
#[derive(Debug, Clone, Default)]
pub struct SlashMenuBuilder {
    items: Vec<MenuItem>,
    query: String,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl SlashMenuBuilder {
    /// Replace the command list.
    #[must_use]
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = MenuItem>,
    {
        self.items = items.into_iter().collect();
        self
    }

    /// Add a single command.
    #[must_use]
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Populate items with the Claude defaults.
    pub fn with_defaults(mut self) -> Result<Self> {
        self.items = default_commands()?;
        Ok(self)
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

    /// Set whether the adapter should draw a scrollbar indicator.
    #[must_use]
    pub const fn show_scrollbar(mut self, on: bool) -> Self {
        self.show_scrollbar = on;
        self
    }

    /// Override the theme.
    #[must_use]
    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Build the menu. Errors if no items were added.
    pub fn build(self) -> Result<SlashMenu> {
        if self.items.is_empty() {
            return Err(Error::EmptyMenu);
        }
        let count = self.items.len();
        let mut slash = SlashMenu {
            items: self.items,
            filtered: (0..count).collect(),
            query: self.query,
            selection: Selection::new(count),
            theme: self.theme,
            show_scrollbar: self.show_scrollbar,
        };
        if !slash.query.is_empty() {
            slash.recompute_filtered();
        }
        Ok(slash)
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
    fn defaults_have_six_commands() -> Result<()> {
        let m = SlashMenu::with_defaults()?;
        assert_eq!(m.items().len(), 6);
        Ok(())
    }

    #[test]
    fn typing_filters() -> Result<()> {
        let mut m = SlashMenu::with_defaults()?;
        let _ = m.update(ev(KeyCode::Char('a')));
        let _ = m.update(ev(KeyCode::Char('g')));
        assert_eq!(m.query(), "ag");
        assert_eq!(m.filtered_indices().len(), 1);
        Ok(())
    }

    #[test]
    fn backspace_unfilters() -> Result<()> {
        let mut m = SlashMenu::with_defaults()?;
        let _ = m.update(ev(KeyCode::Char('a')));
        let _ = m.update(ev(KeyCode::Char('g')));
        let _ = m.update(ev(KeyCode::Backspace));
        let _ = m.update(ev(KeyCode::Backspace));
        assert_eq!(m.filtered_indices().len(), 6);
        Ok(())
    }

    #[test]
    fn enter_returns_original_item() -> Result<()> {
        let mut m = SlashMenu::with_defaults()?;
        let _ = m.update(ev(KeyCode::Char('c')));
        let _ = m.update(ev(KeyCode::Char('o')));
        let action = m.update(ev(KeyCode::Enter));
        assert!(matches!(action, MenuAction::Selected(ref i) if i.name == "/compact"));
        Ok(())
    }

    #[test]
    fn filter_is_case_insensitive() -> Result<()> {
        let mut m = SlashMenu::with_defaults()?;
        let _ = m.update(ev(KeyCode::Char('A')));
        let _ = m.update(ev(KeyCode::Char('G')));
        assert_eq!(m.filtered_indices().len(), 1);
        Ok(())
    }

    #[test]
    fn empty_filter_shows_all() -> Result<()> {
        let m = SlashMenu::with_defaults()?;
        assert_eq!(m.filtered_indices().len(), m.items().len());
        Ok(())
    }

    #[test]
    fn show_scrollbar_builder_default_is_false() -> Result<()> {
        let m = SlashMenu::with_defaults()?;
        assert!(!m.show_scrollbar());
        let m = SlashMenu::builder()
            .with_defaults()?
            .show_scrollbar(true)
            .build()?;
        assert!(m.show_scrollbar());
        Ok(())
    }
}
