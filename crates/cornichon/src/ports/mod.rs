//! Consumer-facing ports — the types consumers need to type-annotate when
//! using cornichon.
//!
//! Per the pragmatic trait-split decision, cornichon depends directly on
//! `ratatui` and `crossterm` rather than abstracting them behind consumer-
//! owned traits. The "ports" in this crate are therefore *type re-exports*
//! rather than trait interfaces: consumers don't need to add ratatui /
//! crossterm as direct dependencies to write
//!
//! ```rust,ignore
//! fn handle(event: cornichon::ports::KeyEvent) { ... }
//! ```
//!
//! but they only need to if they want to *construct* events themselves
//! (most users just forward events from `crossterm::event::read`).
//!
//! # Layout note
//!
//! If we ever move to a strict consumer-owned-traits design (one render
//! trait per menu, owned by the application layer), the `ports::render`
//! module is where those traits will live. The current shape is the
//! pragmatic minimum that still respects the
//! `src/{domain,ports,application,adapters}` layout standard.

pub mod input;
pub mod menu_event;
pub mod render;

#[cfg(feature = "async")]
pub mod async_support;

pub use input::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
pub use menu_event::{MenuDispatch, MenuEvent, dispatch};
pub use render::{Buffer, Rect, RenderTarget, Style, Widget};

#[cfg(feature = "async")]
pub use async_support::drive_until_action;
