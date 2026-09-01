//! [`PermissionPrompt`] — Claude Code's "Do you want to proceed?" chooser.
//!
//! A [`SelectMenu`] variant with a fixed header (title, command, question)
//! and pre-attached numeric hotkeys (`1.`, `2.`, `3.`). The defaults match
//! the brainless capture exactly:
//!
//! 1. Yes
//! 2. Yes, and don't ask again this session
//! 3. No, and tell Claude what to do (esc)

use crate::application::menu::common::handle_common_key;
use crate::domain::{MenuAction, MenuItem, Result, Selection, Theme};

/// Default options for a Claude Code permission prompt.
///
/// Matches the brainless capture verbatim.
pub const DEFAULT_OPTIONS: &[(&str, &str)] = &[
    ("Yes", "1"),
    ("Yes, and don't ask again this session", "2"),
    ("No, and tell Claude what to do (esc)", "3"),
];

/// Parse the [`DEFAULT_OPTIONS`] table into typed [`MenuItem`]s.
///
/// Exposed so callers can build their own prompt variants from the same
/// defaults. Returns an error if any default entry fails boundary
/// validation (which would only happen if a future edit introduces a
/// malformed label).
pub fn default_options() -> Result<Vec<MenuItem>> {
    DEFAULT_OPTIONS
        .iter()
        .map(|(label, key)| {
            let key_char = key.chars().next();
            MenuItem::try_from((String::from(*label), String::new())).map(|i| match key_char {
                Some(c) => i.with_key(c),
                None => i,
            })
        })
        .collect()
}

/// A "Do you want to proceed?" chooser.
///
/// # Examples
///
/// ```rust
/// use cornichon::prelude::*;
///
/// let mut prompt = PermissionPrompt::with_defaults()?;
///
/// // Press '2' to jump to "Yes, and don't ask again".
/// let _ = prompt.update(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
/// let action = prompt.update(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
/// assert!(matches!(action, MenuAction::Selected(_)));
/// # Ok::<(), cornichon::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct PermissionPrompt {
    title: String,
    command: String,
    question: String,
    items: Vec<MenuItem>,
    selection: Selection,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl PermissionPrompt {
    /// Start a builder.
    #[must_use]
    pub fn builder() -> PermissionPromptBuilder {
        PermissionPromptBuilder::default()
    }

    /// Convenience: builder with the Claude defaults populated.
    ///
    /// Equivalent to `PermissionPrompt::builder().with_defaults()?.build()`.
    pub fn with_defaults() -> Result<Self> {
        Self::builder().with_defaults()?.build()
    }

    /// The title shown above the prompt.
    #[must_use]
    pub const fn title(&self) -> &str {
        // `String::as_str` is const; `&self.title` would need `Deref`.
        self.title.as_str()
    }

    /// The command (or operation) being approved.
    #[must_use]
    pub const fn command(&self) -> &str {
        self.command.as_str()
    }

    /// The question shown to the user.
    #[must_use]
    pub const fn question(&self) -> &str {
        self.question.as_str()
    }

    /// The selectable options.
    #[must_use]
    pub const fn items(&self) -> &[MenuItem] {
        // `Vec::as_slice` is const; `&self.items[..]` would need `Index`.
        self.items.as_slice()
    }

    /// The currently-selected index.
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

    /// Apply a key event.
    pub fn update(&mut self, event: crate::ports::KeyEvent) -> MenuAction {
        handle_common_key(&mut self.selection, &self.items, event)
    }

    /// Dispatch a unified [`crate::ports::MenuEvent`] (keyboard OR mouse)
    /// to this menu. Default `on_mouse` returns [`MenuAction::None`].
    pub fn dispatch(&mut self, event: impl Into<crate::ports::MenuEvent>) -> crate::MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl crate::ports::MenuDispatch for PermissionPrompt {
    fn key_input(&mut self, event: crate::ports::KeyEvent) -> crate::MenuAction {
        self.update(event)
    }

    fn on_mouse(&mut self, event: crate::ports::MouseEvent) -> crate::MenuAction {
        use crate::ports::MouseEventKind;
        match event.kind {
            MouseEventKind::Down(crate::ports::MouseButton::Left) => {
                let row = event.row as usize;
                match self.items.get(row) {
                    Some(item) => {
                        self.selection.set(row);
                        crate::MenuAction::Selected(item.clone())
                    }
                    None => crate::MenuAction::None,
                }
            }
            _ => crate::MenuAction::None,
        }
    }
}

/// Builder for [`PermissionPrompt`].
///
/// Defaults match the brainless capture: title `"Bash command"`, command
/// `"<unknown>"`, question `"Do you want to proceed?"`, three options with
/// hotkeys `1`/`2`/`3`.
#[derive(Debug, Clone)]
pub struct PermissionPromptBuilder {
    title: String,
    command: String,
    question: String,
    items: Vec<MenuItem>,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl Default for PermissionPromptBuilder {
    fn default() -> Self {
        // Start with empty items; callers opt into defaults via `.with_defaults()`
        // so any future malformed default label surfaces as a typed error.
        Self {
            title: "Bash command".to_owned(),
            command: "<unknown>".to_owned(),
            question: "Do you want to proceed?".to_owned(),
            items: Vec::new(),
            theme: Theme::default(),
            show_scrollbar: false,
        }
    }
}

impl PermissionPromptBuilder {
    /// Override the title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Override the command being approved.
    #[must_use]
    pub fn command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }

    /// Override the question text.
    #[must_use]
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.question = question.into();
        self
    }

    /// Populate items with the Claude defaults (1/Yes, 2/Yes-always, 3/No).
    ///
    /// Propagates any parsing error via [`Result`].
    pub fn with_defaults(mut self) -> Result<Self> {
        self.items = default_options()?;
        Ok(self)
    }

    /// Replace the default options with a custom set.
    #[must_use]
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = MenuItem>,
    {
        self.items = items.into_iter().collect();
        self
    }

    /// Add a single option.
    #[must_use]
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Override the theme.
    #[must_use]
    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set whether the adapter should draw a scrollbar indicator.
    #[must_use]
    pub const fn show_scrollbar(mut self, on: bool) -> Self {
        self.show_scrollbar = on;
        self
    }

    /// Build the prompt. Errors if no options were added.
    pub fn build(self) -> Result<PermissionPrompt> {
        use crate::domain::error::Error;
        if self.items.is_empty() {
            return Err(Error::EmptyMenu);
        }
        let count = self.items.len();
        Ok(PermissionPrompt {
            title: self.title,
            command: self.command,
            question: self.question,
            items: self.items,
            selection: Selection::new(count),
            theme: self.theme,
            show_scrollbar: self.show_scrollbar,
        })
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
    fn defaults_to_three_options() -> Result<()> {
        let p = PermissionPrompt::with_defaults()?;
        assert_eq!(p.items().len(), 3);
        Ok(())
    }

    #[test]
    fn hotkey_jumps_to_choice() -> Result<()> {
        let mut p = PermissionPrompt::with_defaults()?;
        let _ = p.update(ev(KeyCode::Char('2')));
        assert_eq!(p.selection(), 1);
        let action = p.update(ev(KeyCode::Enter));
        assert!(matches!(action, MenuAction::Selected(ref i) if i.key == Some('2')));
        Ok(())
    }

    #[test]
    fn esc_cancels() -> Result<()> {
        let mut p = PermissionPrompt::with_defaults()?;
        assert_eq!(p.update(ev(KeyCode::Esc)), MenuAction::Cancelled);
        Ok(())
    }
}
