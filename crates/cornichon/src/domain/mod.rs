//! Domain layer for the cornichon menu library.
//!
//! Contains pure value objects, enums, and primitives. Nothing in this module
//! (or its submodules) depends on `ratatui`, `crossterm`, or any other
//! infrastructure crate — that isolation is what lets the state machines in
//! [`crate::application`] be tested without a terminal.
//!
//! Following the collapsed-hexagonal layout standard
//! (`src/{domain,ports,application,adapters}`), the domain is the innermost
//! ring: no outward dependencies, only [`crate::ports`] for re-exports of
//! third-party types at the consumer-facing boundary.
//!
//! # Modules
//!
//! - [`item`]: [`MenuItem`](item::MenuItem) and [`AnnotatedItem`](item::AnnotatedItem) — parsed
//!   value objects built from raw `&str` input via [`TryFrom`].
//! - [`action`]: [`MenuAction`](action::MenuAction) — what a menu did in response to an event.
//! - [`theme`]: [`Theme`](theme::Theme) and the bundled `Claude`, `Codex`, `Grok` color sets.
//! - [`selection`]: [`Selection`](selection::Selection) — Copy cursor primitive shared by every menu.
//! - [`error`]: [`Error`](error::Error) — the single library-wide error type.

pub mod action;
pub mod error;
pub mod forms;
pub mod item;
pub mod selection;
pub mod spinner;
pub mod theme;

pub use action::MenuAction;
pub use error::{Error, Result};
pub use item::{AnnotatedItem, MenuItem};
pub use selection::Selection;
pub use spinner::{Cadence, SpinnerStyle, Tick, UnknownSpinnerError};
pub use theme::{CLAUDE, CODEX, GROK, Theme};
