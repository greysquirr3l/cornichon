//! Re-exports of `crossterm` key + mouse types so consumers don't need a
//! direct `crossterm` dependency just to type-annotate event handlers.
//!
//! This is the *pragmatic* shape (per the trait-split decision logged in
//! `AGENTS.md`): rather than abstracting `KeyEvent` / `MouseEvent`
//! into cornichon-owned enums, we re-export the underlying crossterm
//! types. The trade-off is that consumers using `termion` / `termwiz`
//! will need to convert their events to
//! `crossterm::event::{KeyEvent, MouseEvent}` before forwarding.
//!
//! See [`crate::ports::menu_event::MenuEvent`] for a unifying wrapper
//! that callers can use to dispatch both keyboard and mouse input to
//! any menu widget via a single `dispatch(menu, event)` call.

pub use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
