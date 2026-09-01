//! Menu item value objects.
//!
//! These are the only types that flow into a menu's item list. Every other
//! type in the library operates on `MenuItem` / `AnnotatedItem` — there is no
//! path from raw `&str` to a rendered row that bypasses the `TryFrom`
//! boundary, per the parse-don't-validate standard.

use crate::domain::error::{Error, Result};

/// A menu option: a `name`, an optional `description`, and an optional
/// single-character hotkey.
///
/// `name` is the primary displayed string (e.g. `"/agents"`,
/// `"Yes, and don't ask again"`). `description` is the muted secondary
/// string shown after a fixed-width gutter. `key` is the hotkey used by
/// `PermissionPrompt`, `ProjectPicker`, and any menu that wants
/// `1`/`2`/`3` direct-selection.
///
/// # Construction
///
/// ```rust
/// use cornichon::domain::item::MenuItem;
///
/// let item = MenuItem::try_from(("/agents", "Manage subagents"))?;
/// assert_eq!(item.name, "/agents");
/// # Ok::<(), cornichon::domain::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuItem {
    /// Displayed as the primary label.
    pub name: String,
    /// Displayed muted, after a fixed-width gutter.
    pub description: String,
    /// Optional single-character hotkey (digit, letter, etc).
    pub key: Option<char>,
}

impl MenuItem {
    /// Attach a hotkey to this item.
    #[must_use]
    pub const fn with_key(mut self, key: char) -> Self {
        self.key = Some(key);
        self
    }

    /// Borrow the name without cloning.
    #[must_use]
    pub const fn name(&self) -> &str {
        // `String::as_str` is const; `&self.name` would need `Deref`.
        self.name.as_str()
    }

    /// Borrow the description without cloning.
    #[must_use]
    pub const fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// Parse a `(name, description)` pair into a [`MenuItem`].
///
/// Returns [`Error::EmptyName`] if `name` is empty and
/// [`Error::NameHasNewline`] if `name` contains a `\n`. The description is
/// allowed to be empty (it just renders as nothing) and is allowed to
/// contain newlines (it gets truncated for display, never split across rows).
///
/// This is the *only* path from raw `&str` into a [`MenuItem`] — every
/// adapter that takes user input should funnel through here.
impl TryFrom<(&str, &str)> for MenuItem {
    type Error = Error;

    fn try_from((name, description): (&str, &str)) -> Result<Self> {
        Self::parse_str(name, description)
    }
}

/// Owned-string variant of the parser.
impl TryFrom<(String, String)> for MenuItem {
    type Error = Error;

    fn try_from((name, description): (String, String)) -> Result<Self> {
        Self::parse_str(&name, &description)
    }
}

impl MenuItem {
    fn parse_str(name: &str, description: &str) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }
        if name.contains('\n') {
            return Err(Error::NameHasNewline(name.to_owned()));
        }
        Ok(Self {
            name: name.to_owned(),
            description: description.to_owned(),
            key: None,
        })
    }
}

/// A [`MenuItem`] with an additional right-aligned annotation column.
///
/// Used by [`PickerMenu`](crate::application::menu::picker::PickerMenu) for
/// rows like:
/// ```text
/// brainless    ~/dev/brainless           (current)
/// conduit      ~/Documents/Conduit       (5h ago)
/// ```
///
/// The `annotation` is rendered dimmed; if it contains a newline the
/// constructor rejects it for the same reason [`MenuItem`] does.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnnotatedItem {
    /// Primary label — same semantics as [`MenuItem::name`].
    pub name: String,
    /// Secondary muted text — same semantics as [`MenuItem::description`].
    pub description: String,
    /// Optional right-aligned annotation (path, timestamp, badge).
    pub annotation: Option<String>,
    /// Optional single-character hotkey.
    pub key: Option<char>,
}

impl AnnotatedItem {
    /// Attach a hotkey to this item.
    #[must_use]
    pub const fn with_key(mut self, key: char) -> Self {
        self.key = Some(key);
        self
    }
}

impl TryFrom<(&str, &str, Option<&str>)> for AnnotatedItem {
    type Error = Error;

    fn try_from((name, description, annotation): (&str, &str, Option<&str>)) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }
        if name.contains('\n') {
            return Err(Error::NameHasNewline(name.to_owned()));
        }
        let annotation = annotation
            .map(str::to_owned)
            .map(|s| {
                if s.contains('\n') {
                    Err(Error::AnnotationHasNewline(s))
                } else {
                    Ok(s)
                }
            })
            .transpose()?;
        Ok(Self {
            name: name.to_owned(),
            description: description.to_owned(),
            annotation,
            key: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_pair() -> Result<()> {
        let item = MenuItem::try_from(("/clear", "Clear conversation history"))?;
        assert_eq!(item.name, "/clear");
        assert_eq!(item.description, "Clear conversation history");
        assert_eq!(item.key, None);
        Ok(())
    }

    #[test]
    fn rejects_empty_name() {
        let result = MenuItem::try_from(("", "x"));
        assert!(matches!(result, Err(Error::EmptyName)));
    }

    #[test]
    fn rejects_newline_in_name() {
        let result = MenuItem::try_from(("/bad\nname", "x"));
        assert!(matches!(result, Err(Error::NameHasNewline(_))));
    }

    #[test]
    fn with_key_attaches_hotkey() -> Result<()> {
        let item = MenuItem::try_from(("yes", "approve"))?.with_key('y');
        assert_eq!(item.key, Some('y'));
        Ok(())
    }

    #[test]
    fn annotated_accepts_no_annotation() -> Result<()> {
        let item = AnnotatedItem::try_from(("name", "desc", None))?;
        assert_eq!(item.annotation, None);
        Ok(())
    }

    #[test]
    fn annotated_rejects_newline_in_annotation() {
        let result = AnnotatedItem::try_from(("name", "desc", Some("bad\nannotation")));
        assert!(matches!(result, Err(Error::AnnotationHasNewline(_))));
    }
}
