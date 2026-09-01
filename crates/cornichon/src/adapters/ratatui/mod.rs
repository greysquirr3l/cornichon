//! ratatui adapter — registers `Widget` impls on `&Menu` for each menu type.
//!
//! Consumers don't need to import from this module directly: they call
//! `frame.render_widget(&menu, area)` and the appropriate impl runs.
//! This module exists so that the rendering logic can be unit-tested in
//! isolation against a fresh `Buffer`.

pub mod choice;
pub mod common;
pub mod forms;
pub mod picker;
pub mod select;
pub mod shortcuts;
pub mod slash;
pub mod spinner;
