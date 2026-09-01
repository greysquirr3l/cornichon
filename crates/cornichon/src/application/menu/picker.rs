//! [`ProjectPicker`] — Grok CLI's "Run in a project directory?" chooser.
//!
//! Like a [`SelectMenu`](crate::application::menu::select::SelectMenu) but
//! with a *trailing* "custom" option that's distinguished in the action
//! returned: choosing any project returns [`MenuAction::Selected`] (with
//! the project item), choosing the custom option returns
//! [`MenuAction::Activated`] (signaling the caller to collect free-text
//! input instead).
//!
//! Items are [`AnnotatedItem`](crate::domain::item::AnnotatedItem)s so they
//! can carry a path + meta annotation column. Hotkeys default to `1`, `2`,
//! `3`, ... for the projects and `z` for the custom option (matching the
//! brainless capture).

use crate::application::menu::common::handle_common_key;
use crate::domain::{Error, MenuAction, MenuItem, Result, Selection, Theme, item::AnnotatedItem};
use crate::ports::KeyEvent;

/// Default projects for the picker — matches the brainless capture.
pub const DEFAULT_PROJECTS: &[(&str, &str, &str, &str)] = &[
    ("current", "brainless", "~/dev/brainless", "current"),
    (
        "cloudstate",
        "freestyle-cloudstate",
        "~/Documents/GitHub/freestyle-cloudstate",
        "5h ago",
    ),
    ("conduit", "Conduit", "~/Documents/Conduit", "5h ago"),
];

/// Hotkey for the trailing "custom" option.
const CUSTOM_KEY: char = 'z';

/// Parse [`DEFAULT_PROJECTS`] into typed [`AnnotatedItem`]s.
///
/// Exposed so callers can build their own picker variants from the same
/// defaults. Returns an error if any default entry fails boundary
/// validation (which would only happen if a future edit introduces a
/// malformed entry).
pub fn default_projects() -> Result<Vec<AnnotatedItem>> {
    DEFAULT_PROJECTS
        .iter()
        .enumerate()
        .map(|(i, (_id, name, path, meta))| {
            let key = u32::try_from(i + 1)
                .ok()
                .and_then(|n| char::from_digit(n, 10));
            let annotation = if *meta == "current" {
                None
            } else {
                Some(*meta)
            };
            AnnotatedItem::try_from((*name, *path, annotation)).map(|item| match key {
                Some(c) => item.with_key(c),
                None => item,
            })
        })
        .collect()
}

/// A single-select list with a trailing "custom" option.
///
/// Selecting any project yields [`MenuAction::Selected`] with the project
/// as a [`MenuItem`]. Selecting the custom option yields
/// [`MenuAction::Activated`] (the caller should then collect free-text
/// input).
///
/// # Examples
///
/// ```rust
/// use cornichon::prelude::*;
///
/// let mut picker = ProjectPicker::with_defaults()?;
///
/// // Press '1' to select the first project.
/// let _ = picker.update(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));
/// let action = picker.update(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
/// assert!(matches!(action, MenuAction::Selected(_)));
/// # Ok::<(), cornichon::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct ProjectPicker {
    title: String,
    description: String,
    projects: Vec<AnnotatedItem>,
    custom_label: String,
    selection: Selection,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl ProjectPicker {
    /// Start a builder.
    #[must_use]
    pub fn builder() -> ProjectPickerBuilder {
        ProjectPickerBuilder::default()
    }

    /// Convenience: builder with the Grok defaults populated.
    ///
    /// Equivalent to `ProjectPicker::builder().with_defaults()?.build()`.
    pub fn with_defaults() -> Result<Self> {
        Self::builder().with_defaults()?.build()
    }

    /// The title shown above the list.
    #[must_use]
    pub const fn title(&self) -> &str {
        self.title.as_str()
    }

    /// The description shown between the title and the list.
    #[must_use]
    pub const fn description(&self) -> &str {
        self.description.as_str()
    }

    /// The project entries (excluding the trailing "custom" option).
    #[must_use]
    pub const fn projects(&self) -> &[AnnotatedItem] {
        self.projects.as_slice()
    }

    /// The label rendered for the trailing "custom" option.
    #[must_use]
    pub const fn custom_label(&self) -> &str {
        self.custom_label.as_str()
    }

    /// Currently-selected index (covers both projects and the custom option).
    #[must_use]
    pub const fn selection(&self) -> usize {
        self.selection.index()
    }

    /// Whether the current selection is the trailing "custom" option.
    #[must_use]
    pub const fn is_custom_selected(&self) -> bool {
        self.selection.index() == self.projects.len()
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
    /// Selecting the custom option returns [`MenuAction::Activated`]
    /// rather than [`MenuAction::Selected`], so the caller can dispatch
    /// to free-text input collection.
    pub fn update(&mut self, event: KeyEvent) -> MenuAction {
        let items = self.items_view();
        match handle_common_key(&mut self.selection, &items, event) {
            MenuAction::Selected(_) if self.is_custom_selected() => MenuAction::Activated,
            other => other,
        }
    }

    /// Dispatch a unified [`crate::ports::MenuEvent`] (keyboard OR mouse)
    /// to this menu. Default `on_mouse` returns [`MenuAction::None`].
    pub fn dispatch(&mut self, event: impl Into<crate::ports::MenuEvent>) -> MenuAction {
        crate::ports::dispatch(self, event)
    }

    /// Build the items list as it appears to `update()` — projects plus
    /// the trailing "custom" option.
    fn items_view(&self) -> Vec<MenuItem> {
        let mut v: Vec<MenuItem> = self
            .projects
            .iter()
            .map(|p| MenuItem {
                name: p.name.clone(),
                description: p.description.clone(),
                key: p.key,
            })
            .collect();
        v.push(MenuItem {
            name: self.custom_label.clone(),
            description: String::new(),
            key: Some(CUSTOM_KEY),
        });
        v
    }
}

impl crate::ports::MenuDispatch for ProjectPicker {
    fn key_input(&mut self, event: KeyEvent) -> MenuAction {
        self.update(event)
    }

    fn on_mouse(&mut self, event: crate::ports::MouseEvent) -> MenuAction {
        use crate::ports::MouseEventKind;
        match event.kind {
            MouseEventKind::Down(crate::ports::MouseButton::Left) => {
                let row = event.row as usize;
                if row < self.projects.len() + 1 {
                    self.selection.set(row);
                }
                // Clicking Enter would do — dispatching the same event through
                // `update()` ensures keyboard parity.
                self.update(crate::ports::KeyEvent::new(
                    crate::ports::KeyCode::Enter,
                    crate::ports::KeyModifiers::NONE,
                ))
            }
            _ => MenuAction::None,
        }
    }
}

/// Builder for [`ProjectPicker`].
#[derive(Debug, Clone)]
pub struct ProjectPickerBuilder {
    title: String,
    description: String,
    projects: Vec<AnnotatedItem>,
    custom_label: String,
    theme: Theme,
    /// Whether the adapter should draw a scrollbar indicator at the right
    /// edge of the visible list. Default `false`.
    show_scrollbar: bool,
}

impl Default for ProjectPickerBuilder {
    fn default() -> Self {
        // Start with empty projects; callers opt into defaults via
        // `.with_defaults()` so any future malformed default entry surfaces
        // as a typed error.
        Self {
            title: "Run in a project directory?".to_owned(),
            description: "This gives full context of your codebase for better results.".to_owned(),
            projects: Vec::new(),
            custom_label: "Type your answer here".to_owned(),
            theme: Theme::default(),
            show_scrollbar: false,
        }
    }
}

impl ProjectPickerBuilder {
    /// Override the title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Override the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Replace the project entries.
    #[must_use]
    pub fn projects<I>(mut self, projects: I) -> Self
    where
        I: IntoIterator<Item = AnnotatedItem>,
    {
        self.projects = projects.into_iter().collect();
        self
    }

    /// Add a single project.
    #[must_use]
    pub fn project(mut self, project: AnnotatedItem) -> Self {
        self.projects.push(project);
        self
    }

    /// Override the trailing "custom" option's label.
    #[must_use]
    pub fn custom_label(mut self, label: impl Into<String>) -> Self {
        self.custom_label = label.into();
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

    /// Populate projects with the Grok defaults.
    ///
    /// Propagates any parsing error via [`Result`].
    pub fn with_defaults(mut self) -> Result<Self> {
        self.projects = default_projects()?;
        Ok(self)
    }

    /// Build the picker. Errors if no projects were added or if the
    /// custom label is empty.
    pub fn build(self) -> Result<ProjectPicker> {
        if self.projects.is_empty() {
            return Err(Error::EmptyMenu);
        }
        if self.custom_label.is_empty() {
            return Err(Error::EmptyCustomLabel);
        }
        let count = self.projects.len() + 1; // +1 for the custom option
        Ok(ProjectPicker {
            title: self.title,
            description: self.description,
            projects: self.projects,
            custom_label: self.custom_label,
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
    fn defaults_have_three_projects() -> Result<()> {
        let p = ProjectPicker::with_defaults()?;
        assert_eq!(p.projects().len(), 3);
        Ok(())
    }

    #[test]
    fn selecting_project_returns_selected() -> Result<()> {
        let mut p = ProjectPicker::with_defaults()?;
        let _ = p.update(ev(KeyCode::Char('1')));
        let action = p.update(ev(KeyCode::Enter));
        assert!(matches!(action, MenuAction::Selected(_)));
        Ok(())
    }

    #[test]
    fn selecting_custom_returns_activated() -> Result<()> {
        let mut p = ProjectPicker::with_defaults()?;
        let _ = p.update(ev(KeyCode::Char('z')));
        assert!(p.is_custom_selected());
        let action = p.update(ev(KeyCode::Enter));
        assert_eq!(action, MenuAction::Activated);
        Ok(())
    }

    #[test]
    fn rejects_empty_custom_label() -> Result<()> {
        // Start from defaults so the empty-custom-label check is reached
        // (the default builder has no projects, so without `.with_defaults()`
        // the empty-projects check fires first).
        let result = ProjectPicker::builder()
            .with_defaults()?
            .custom_label("")
            .build();
        assert!(matches!(result, Err(Error::EmptyCustomLabel)));
        Ok(())
    }
}
