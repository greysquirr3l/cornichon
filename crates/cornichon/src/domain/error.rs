//! Library-wide error type.
//!
//! Per `~/Projects/nick.md` ("Library crates: `thiserror`"), cornichon uses an
//! explicit error enum so consumers can match on failure modes rather than
//! receiving an erased `Box<dyn Error>`.
//!
//! All errors are constructed from `MenuItem` boundary parsing — the rest of
//! the library is total (returns `Option` or `MenuAction::None` instead of
//! `Result`) so most consumers never need to handle [`Error`] at all.

use thiserror::Error;

/// Result alias for the cornichon crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors produced at value-object construction boundaries.
///
/// Per the parse-don't-validate standard, raw `&str` / `String` input is
/// converted into a [`MenuItem`](crate::domain::item::MenuItem) (or
/// [`AnnotatedItem`](crate::domain::item::AnnotatedItem)) once, at the
/// adapter boundary, and the validated type is what flows through the rest of
/// the system.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// A menu item's `name` field was empty.
    ///
    /// Empty names break filter matching and selection-by-hotkey, so we refuse
    /// to construct them.
    #[error("menu item name cannot be empty")]
    EmptyName,

    /// A menu item's `name` contained a newline.
    ///
    /// Newlines in a name would split the rendered row and corrupt the layout
    /// of every menu in the library.
    #[error("menu item name cannot contain a newline: {0:?}")]
    NameHasNewline(String),

    /// A menu item's `annotation` contained a newline.
    ///
    /// Same reasoning as [`Error::NameHasNewline`], applied to the trailing
    /// annotation column used by [`AnnotatedItem`](crate::domain::item::AnnotatedItem).
    #[error("menu item annotation cannot contain a newline: {0:?}")]
    AnnotationHasNewline(String),

    /// A menu was constructed with zero items.
    ///
    /// An empty menu has no selectable option and no meaningful default
    /// cursor position, so we refuse the construction rather than silently
    /// shipping a no-op widget.
    #[error("menu must have at least one item")]
    EmptyMenu,

    /// A custom option for a [`PickerMenu`](crate::application::menu::picker::PickerMenu) was empty.
    #[error("picker custom-option label cannot be empty")]
    EmptyCustomLabel,
}
