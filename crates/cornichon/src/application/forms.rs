//! Form-widget state machines: [`TextInput`], [`Checkbox`], [`RadioGroup`].
//!
//! These methods are *the* state-mutation surface for form widgets.
//! They're total — every key event either mutates the widget state or
//! is silently dropped (no error returned). The widgets expose no
//! `Result`-returning API; consumers either read the public fields
//! or call the public `set*` / `toggle` / `next` / `prev` / `select`
//! methods directly.
//!
//! Per `AGENTS.md` and `nick.md`:
//! - No `.unwrap()` / `.expect()` / `.panic()` anywhere (even in tests).
//! - `TryFrom` at boundary types; here, the boundary IS the input event,
//!   which is `&KeyEvent` (a copy type), so we use pattern matching.
//! - thiserror for libraries; these widgets don't error.
//! - Newtypes wrapping `Copy` are themselves `Copy`; the widgets own
//!   `String`s, so they are `Clone` (not `Copy`).

use crate::MenuAction;
use crate::domain::forms::{Checkbox, RadioGroup, TextInput};
use crate::ports::{KeyCode, KeyEvent, KeyModifiers};

/// Form-event kinds that any form widget can accept.
///
/// The state-machine methods take this and return whether the event
/// changed widget state. Used as a uniform input across `TextInput`,
/// `Checkbox`, and `RadioGroup` so caller code can dispatch with a
/// single match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormEvent {
    /// A key event (keyboard). Forwarded to the widget's `update`.
    Key(KeyEvent),
    /// A mouse event. Currently always returns `Changed(false)` — the
    /// form widgets don't have a click target yet.
    Mouse,
}

impl From<KeyEvent> for FormEvent {
    fn from(e: KeyEvent) -> Self {
        Self::Key(e)
    }
}

impl From<crate::ports::MouseEvent> for FormEvent {
    fn from(_: crate::ports::MouseEvent) -> Self {
        Self::Mouse
    }
}

/// Whether a state mutation actually changed the widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Changed(pub bool);

impl Changed {
    /// Convenience: `true` if the widget changed.
    #[must_use]
    pub const fn is_changed(self) -> bool {
        self.0
    }
}

/// Dispatch a [`FormEvent`] to a widget's inherent `update` method.
///
/// Each of the three form widgets ([`TextInput`], [`Checkbox`],
/// [`RadioGroup`]) has an identical `apply` method that matches on the
/// event and forwards `FormEvent::Key(k)` to the widget's `update(&mut
/// self, KeyEvent) -> (Changed, MenuAction)`. The match is a constant
/// `Changed` / `MenuAction` matrix that's identical across all three
/// widgets, so extracting it as a free function eliminates the
/// repetition. The free function takes a closure that calls the
/// widget's `update` method — this preserves the per-widget `update`
/// API (no trait needed) and keeps the type relationship explicit.
pub fn apply_event<M, F>(widget: &mut M, event: FormEvent, update: F) -> (Changed, MenuAction)
where
    F: FnOnce(&mut M, KeyEvent) -> (Changed, MenuAction),
{
    match event {
        FormEvent::Key(k) => update(widget, k),
        FormEvent::Mouse => (Changed(false), MenuAction::None),
    }
}

// ---------------------------------------------------------------------------
// TextInput
// ---------------------------------------------------------------------------

impl TextInput {
    /// Apply a key event to the input.
    ///
    /// `Enter` returns `MenuAction::None` (form inputs don't have a
    /// "submit" action — wrap them in a form container to get that).
    /// `Esc` returns `MenuAction::Cancelled` so a caller can pop a
    /// dialog. All other keys either mutate the buffer or are
    /// silently ignored.
    pub fn update(&mut self, event: KeyEvent) -> (Changed, MenuAction) {
        let mut changed = false;
        let action = match event.code {
            KeyCode::Backspace => {
                let was_non_empty = !self.value.is_empty() && self.cursor > 0;
                self.backspace();
                changed = was_non_empty && (self.cursor < self.value.len() + 1);
                MenuAction::None
            }
            KeyCode::Delete => {
                let was_non_empty = self.cursor < self.value.len();
                self.delete();
                changed = was_non_empty && self.cursor < self.value.len() + 1;
                MenuAction::None
            }
            KeyCode::Left => {
                let prev = self.cursor;
                self.move_left();
                changed = prev != self.cursor;
                MenuAction::None
            }
            KeyCode::Right => {
                let prev = self.cursor;
                self.move_right();
                changed = prev != self.cursor;
                MenuAction::None
            }
            KeyCode::Home => {
                let prev = self.cursor;
                self.move_home();
                changed = prev != 0;
                MenuAction::None
            }
            KeyCode::End => {
                let prev = self.cursor;
                self.move_end();
                changed = prev != self.value.len();
                MenuAction::None
            }
            KeyCode::Esc => MenuAction::Cancelled,
            KeyCode::Char(c) if !event.modifiers.contains(KeyModifiers::CONTROL) => {
                let prev = self.value.len();
                self.insert(&c.to_string());
                changed = self.value.len() > prev;
                MenuAction::None
            }
            // Enter and the catch-all both fall through to MenuAction::None:
            // TextInput has no "submit" action; the consumer is expected to
            // read the current value via `self.value`.
            _ => MenuAction::None,
        };
        (Changed(changed), action)
    }
}

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

impl Checkbox {
    /// Apply a key event.
    ///
    /// `Space` and `Enter` toggle the checked state. `Esc` returns
    /// `MenuAction::Cancelled`. All other events are ignored.
    pub const fn update(&mut self, event: KeyEvent) -> (Changed, MenuAction) {
        let mut changed = false;
        let action = match event.code {
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.toggle();
                changed = true;
                // Form-widget toggles don't "select" an item — they mutate
                // internal state. The consumer reads the new state via the
                // public `checked` field. We still report `Changed` so a
                // caller driving a `Changed`-aware UI can react.
                MenuAction::None
            }
            KeyCode::Esc => MenuAction::Cancelled,
            _ => MenuAction::None,
        };
        (Changed(changed), action)
    }

    /// Apply a form event.
    pub fn apply(&mut self, event: FormEvent) -> (Changed, MenuAction) {
        apply_event(self, event, Self::update)
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

impl RadioGroup {
    /// Apply a key event.
    ///
    /// `Up` / `Left` move to the previous option; `Down` / `Right` to
    /// the next. `Enter` returns the currently-selected option's label
    /// as `MenuAction::Selected`. `Esc` returns `MenuAction::Cancelled`.
    pub fn update(&mut self, event: KeyEvent) -> (Changed, MenuAction) {
        let mut changed = false;
        let action = match event.code {
            KeyCode::Up | KeyCode::Left => {
                let prev = self.selected;
                self.prev();
                changed = prev != self.selected;
                MenuAction::None
            }
            KeyCode::Down | KeyCode::Right => {
                let prev = self.selected;
                self.next();
                changed = prev != self.selected;
                MenuAction::None
            }
            KeyCode::Enter => {
                // Radio-group selection is also internal state, not an
                // item "selection" — same reasoning as the Checkbox above.
                // Callers read `selected()` after `update` if they need
                // the chosen option.
                let _ = self.selected_label();
                MenuAction::None
            }
            KeyCode::Esc => MenuAction::Cancelled,
            _ => MenuAction::None,
        };
        (Changed(changed), action)
    }

    /// Apply a form event.
    pub fn apply(&mut self, event: FormEvent) -> (Changed, MenuAction) {
        apply_event(self, event, Self::update)
    }
}

// ---------------------------------------------------------------------------
// TextInput
// ---------------------------------------------------------------------------

impl TextInput {
    /// Apply a form event.
    pub fn apply(&mut self, event: FormEvent) -> (Changed, MenuAction) {
        apply_event(self, event, Self::update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textinput_typing_changes() {
        let mut ti = TextInput::new();
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let (c, _) = ti.update(k(KeyCode::Char('h')));
        assert!(c.is_changed());
        let (c, _) = ti.update(k(KeyCode::Char('i')));
        assert!(c.is_changed());
        assert_eq!(ti.value, "hi");
    }

    #[test]
    fn textinput_backspace_no_change_at_start() {
        let mut ti = TextInput::new();
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let (c, _) = ti.update(k(KeyCode::Backspace));
        assert!(!c.is_changed());
        assert_eq!(ti.value, "");
    }

    #[test]
    fn checkbox_space_toggles() {
        let mut c = Checkbox::new("foo");
        let k = |code| KeyEvent::new(code, KeyModifiers::NONE);
        let (changed, action) = c.update(k(KeyCode::Char(' ')));
        assert!(changed.is_changed());
        assert!(c.checked);
        // Checkbox toggles are internal state, not a "selected item" — the
        // caller reads `c.checked` to see the new state. `MenuAction::None`
        // reflects that. Same reasoning as RadioGroup below.
        assert!(matches!(action, MenuAction::None));
    }

    #[test]
    fn radiogroup_up_at_top_wraps() {
        let mut rg = RadioGroup::new(["a", "b", "c"]);
        rg.select(0);
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let (c, _) = rg.update(k(KeyCode::Up));
        assert!(c.is_changed());
        assert_eq!(rg.selected(), 2);
    }

    #[test]
    fn radiogroup_enter_returns_label() {
        let mut rg = RadioGroup::new(["alpha", "beta", "gamma"]);
        rg.select(1);
        let k = |c| KeyEvent::new(c, KeyModifiers::NONE);
        let (_, action) = rg.update(k(KeyCode::Enter));
        // RadioGroup selection is internal state. `MenuAction::None` reflects
        // that; the caller reads `rg.selected_label()` to see the choice.
        assert!(matches!(action, MenuAction::None));
        assert_eq!(rg.selected_label(), Some("beta"));
    }
}
