//! [`ShortcutsModal`] — categorized, expandable keyboard-shortcut modal.
//!
//! Like the Grok CLI's Ctrl+x dialog: a list of category headers, one
//! expanded at a time, with the expanded category showing its
//! action/key pairs. Arrow keys navigate between categories; Enter/Right
//! expand; Left collapses (or, if nothing is expanded, returns
//! [`MenuAction::Cancelled`]).

use crate::domain::{MenuAction, Result, Selection, Theme};
use crate::ports::{KeyCode, KeyEvent};

/// Default keyboard-shortcut groups — matches the brainless capture.
pub const DEFAULT_GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "essentials",
        &[
            ("Send", "Enter"),
            ("Focus scrollback", "Tab"),
            ("Cancel turn", "Ctrl+c"),
            ("Cycle mode", "Shift+Tab"),
            ("Quit", "Ctrl+q / Ctrl+d"),
            ("Command palette", "Ctrl+p / ?"),
            ("Keyboard shortcuts", "Ctrl+x / Ctrl+."),
            ("Open the settings modal", "F2 / Ctrl+, / Cmd+,"),
        ],
    ),
    (
        "input",
        &[
            ("Send now while running", "Ctrl+Enter / Ctrl+i"),
            ("Voice dictation", "Ctrl+Space / F8"),
            ("Search prompt history", "Ctrl+r"),
            ("Toggle multiline", "Ctrl+m"),
            ("Shell mode (type ! on empty prompt)", "!"),
        ],
    ),
    (
        "session",
        &[
            ("New session", "Ctrl+n"),
            ("Resume session", "Ctrl+s"),
            ("New worktree", "Ctrl+w"),
        ],
    ),
];

/// A single action/key pair inside an expanded [`ShortcutsGroup`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutsItem {
    /// The action description, e.g. `"Send"`.
    pub action: String,
    /// The key combination, e.g. `"Enter"`.
    pub keys: String,
}

impl ShortcutsItem {
    /// Construct a `ShortcutsItem` from `&str`s.
    pub fn new(action: impl Into<String>, keys: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            keys: keys.into(),
        }
    }
}

/// A named category of shortcuts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutsGroup {
    /// Stable identifier (used to track expansion state).
    pub id: String,
    /// Display label, e.g. `"Essentials"`.
    pub label: String,
    /// Items inside this group.
    pub items: Vec<ShortcutsItem>,
    /// Display count when collapsed; defaults to `items.len()`.
    pub count: Option<usize>,
}

impl ShortcutsGroup {
    /// Construct a `ShortcutsGroup` from owned data.
    pub fn new(id: impl Into<String>, label: impl Into<String>, items: Vec<ShortcutsItem>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            items,
            count: None,
        }
    }

    /// Override the collapsed-mode display count.
    #[must_use]
    pub const fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    /// The count to render when this group is collapsed.
    #[must_use]
    pub const fn collapsed_count(&self) -> usize {
        // `Option::unwrap_or` is not yet stable as const fn; explicit match.
        match self.count {
            Some(c) => c,
            None => self.items.len(),
        }
    }
}

/// Parse [`DEFAULT_GROUPS`] into typed [`ShortcutsGroup`]s.
pub fn default_groups() -> Result<Vec<ShortcutsGroup>> {
    Ok(DEFAULT_GROUPS
        .iter()
        .map(|(id, items)| {
            let items = items
                .iter()
                .map(|(action, keys)| ShortcutsItem::new(*action, *keys))
                .collect();
            ShortcutsGroup::new(*id, *id, items)
        })
        .collect())
}

/// A categorized, expandable shortcuts modal.
///
/// # Examples
///
/// ```rust
/// use cornichon::prelude::*;
///
/// let mut modal = ShortcutsModal::with_defaults()?;
///
/// // Arrow down to move selection, Enter to expand.
/// let _ = modal.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
/// let _ = modal.update(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
///
/// // Esc collapses; second Esc closes the modal.
/// let _ = modal.update(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
/// let action = modal.update(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
/// assert_eq!(action, MenuAction::Cancelled);
/// # Ok::<(), cornichon::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct ShortcutsModal {
    groups: Vec<ShortcutsGroup>,
    selection: Selection,
    expanded: Option<usize>,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl ShortcutsModal {
    /// Start a builder.
    #[must_use]
    pub fn builder() -> ShortcutsModalBuilder {
        ShortcutsModalBuilder::default()
    }

    /// Convenience: builder with the Grok defaults populated.
    pub fn with_defaults() -> Result<Self> {
        Self::builder().with_defaults()?.build()
    }

    /// The shortcut groups, in display order.
    #[must_use]
    pub const fn groups(&self) -> &[ShortcutsGroup] {
        self.groups.as_slice()
    }

    /// Currently-selected group index.
    #[must_use]
    pub const fn selection(&self) -> usize {
        self.selection.index()
    }

    /// The expanded group index, if any.
    #[must_use]
    pub const fn expanded(&self) -> Option<usize> {
        // `Option::as_deref` is not const-stable; explicit match.
        match self.expanded {
            Some(i) => Some(i),
            None => None,
        }
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
    ///
    /// - `Up`/`Down` move the group cursor.
    /// - `Enter`/`Right` expand the current group.
    /// - `Left` collapses the current group (no-op if already collapsed).
    /// - `Esc` collapses first, then on the second press returns
    ///   [`MenuAction::Cancelled`].
    pub const fn update(&mut self, event: KeyEvent) -> MenuAction {
        match event.code {
            KeyCode::Up => {
                self.selection.up();
                MenuAction::None
            }
            KeyCode::Down => {
                self.selection.down();
                MenuAction::None
            }
            KeyCode::Enter | KeyCode::Right => {
                self.expanded = Some(self.selection.index());
                MenuAction::None
            }
            KeyCode::Left => {
                self.expanded = None;
                MenuAction::None
            }
            KeyCode::Esc => {
                if self.expanded.is_some() {
                    self.expanded = None;
                    MenuAction::None
                } else {
                    MenuAction::Cancelled
                }
            }
            _ => MenuAction::None,
        }
    }

    /// Dispatch a unified [`crate::ports::MenuEvent`] (keyboard OR mouse)
    /// to this menu. Default `on_mouse` returns [`MenuAction::None`].
    pub fn dispatch(&mut self, event: impl Into<crate::ports::MenuEvent>) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl crate::ports::MenuDispatch for ShortcutsModal {
    fn key_input(&mut self, event: KeyEvent) -> MenuAction {
        self.update(event)
    }

    fn on_mouse(&mut self, event: crate::ports::MouseEvent) -> MenuAction {
        let _ = event;
        MenuAction::None
    }
}

/// Builder for [`ShortcutsModal`].
#[derive(Debug, Clone, Default)]
pub struct ShortcutsModalBuilder {
    groups: Vec<ShortcutsGroup>,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false` (via `bool::default()`).
    show_scrollbar: bool,
}

impl ShortcutsModalBuilder {
    /// Replace the groups.
    #[must_use]
    pub fn groups<I>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = ShortcutsGroup>,
    {
        self.groups = groups.into_iter().collect();
        self
    }

    /// Add a single group.
    #[must_use]
    pub fn group(mut self, group: ShortcutsGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Populate groups with the Grok defaults.
    pub fn with_defaults(mut self) -> Result<Self> {
        self.groups = default_groups()?;
        Ok(self)
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

    /// Build the modal.
    ///
    /// Unlike other menus, this one tolerates an empty groups list — an
    /// empty modal just renders nothing and every event is `MenuAction::None`.
    /// Callers can choose to surface their own empty-state UX.
    pub fn build(self) -> Result<ShortcutsModal> {
        let count = self.groups.len();
        Ok(ShortcutsModal {
            groups: self.groups,
            selection: Selection::new(count),
            expanded: None,
            theme: self.theme,
            show_scrollbar: self.show_scrollbar,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::KeyModifiers;

    fn ev(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn defaults_have_three_groups() -> Result<()> {
        let m = ShortcutsModal::with_defaults()?;
        assert_eq!(m.groups().len(), 3);
        Ok(())
    }

    #[test]
    fn enter_expands_current_group() -> Result<()> {
        let mut m = ShortcutsModal::with_defaults()?;
        let _ = m.update(ev(KeyCode::Enter));
        assert_eq!(m.expanded(), Some(0));
        Ok(())
    }

    #[test]
    fn left_collapses() -> Result<()> {
        let mut m = ShortcutsModal::with_defaults()?;
        let _ = m.update(ev(KeyCode::Enter));
        let _ = m.update(ev(KeyCode::Left));
        assert_eq!(m.expanded(), None);
        Ok(())
    }

    #[test]
    fn esc_collapses_then_cancels() -> Result<()> {
        let mut m = ShortcutsModal::with_defaults()?;
        let _ = m.update(ev(KeyCode::Enter)); // expand
        assert_eq!(m.update(ev(KeyCode::Esc)), MenuAction::None); // collapse
        assert_eq!(m.update(ev(KeyCode::Esc)), MenuAction::Cancelled); // close
        Ok(())
    }

    #[test]
    fn collapsed_count_defaults_to_items_len() -> Result<()> {
        let m = ShortcutsModal::with_defaults()?;
        let first = m.groups().first();
        match first {
            Some(g) => assert_eq!(g.collapsed_count(), g.items.len()),
            None => return Err(crate::domain::Error::EmptyMenu),
        }
        Ok(())
    }
}
