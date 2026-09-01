//! Unified keyboard + mouse input for menus.
//!
//! `MenuEvent` wraps both [`KeyEvent`] and [`MouseEvent`] so a single
//! dispatch loop can drive a menu regardless of where events came from.
//! Use [`MenuEvent::dispatch`] (or the inherent [`Menu::dispatch`]
//! method) to send an event to any menu widget.
//!
//! [`KeyEvent`]: crate::ports::KeyEvent
//! [`MouseEvent`]: crate::ports::MouseEvent

use crate::MenuAction;
use crate::ports::{KeyEvent, MouseEvent};

/// A menu input event — either a key press or a mouse event.
///
/// Construct via [`From<KeyEvent>`] or [`From<MouseEvent>`]. The
/// conversion is infallible and zero-cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEvent {
    /// A keyboard event (key press, release, or repeat).
    Keyboard(KeyEvent),
    /// A mouse event (move, click, scroll, etc.).
    Mouse(MouseEvent),
}

impl From<KeyEvent> for MenuEvent {
    fn from(event: KeyEvent) -> Self {
        Self::Keyboard(event)
    }
}

impl From<MouseEvent> for MenuEvent {
    fn from(event: MouseEvent) -> Self {
        Self::Mouse(event)
    }
}

/// Dispatch a [`MenuEvent`] to a menu widget built with a standard
/// `update(&mut self, KeyEvent) -> MenuAction` API.
///
/// This is a small convention rather than a formal trait. Each
/// menu widget implements a `dispatch(&mut self, event)` inherent
/// method that delegates mouse events to a hook (`on_mouse`). This
/// free function is provided for callers who'd rather not import
/// the inherent method — e.g., generic event loops.
///
/// ```rust,ignore
/// use cornichon::prelude::*;
///
/// let action = match event {
///     MouseOrKeyboard::Key(k) => menu.update(k),
///     MouseOrKeyboard::Mouse(m) => MenuDispatch::dispatch(&mut menu, m),
/// };
/// ```
pub trait MenuDispatch {
    /// Delegate a keyboard event to the menu's normal `update`.
    fn key_input(&mut self, event: KeyEvent) -> MenuAction;
    /// Handle a mouse event. Defaults to ignoring.
    fn on_mouse(&mut self, _event: MouseEvent) -> MenuAction {
        MenuAction::None
    }
}

/// Dispatch a unified event to a menu implementing [`MenuDispatch`].
///
/// Returns the same [`MenuAction`] the menu would have returned for
/// the corresponding keyboard-only event. Mouse events that the menu
/// doesn't recognize (the default for `on_mouse`) yield `MenuAction::None`.
pub fn dispatch<M: MenuDispatch + ?Sized>(menu: &mut M, event: impl Into<MenuEvent>) -> MenuAction {
    match event.into() {
        MenuEvent::Keyboard(k) => menu.key_input(k),
        MenuEvent::Mouse(m) => menu.on_mouse(m),
    }
}
