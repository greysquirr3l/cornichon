//! Key-handling helpers shared across menus.
//!
//! These are crate-private (`pub(crate)`) — the public update API on each
//! menu calls into them. Kept separate so individual menus can override
//! the bits that differ (e.g. `SlashMenu` routes printable chars into its
//! query field).

use crate::domain::{MenuAction, MenuItem, Selection};
use crate::ports::KeyCode;

/// Apply the common key-handling rules used by every non-filter menu:
/// arrow navigation, Enter to select, Esc to cancel, printable-char
/// hotkey to jump.
///
/// # Returns
///
/// - [`MenuAction::None`] for unhandled events or navigation that doesn't
///   change state visibly.
/// - [`MenuAction::Selected`] on Enter, carrying the item at the current
///   cursor.
/// - [`MenuAction::Cancelled`] on Esc.
/// - Hotkey jumps are silent: the selection moves but no action is
///   returned (the caller will see the new selection on the next render).
pub(crate) fn handle_common_key(
    selection: &mut Selection,
    items: &[MenuItem],
    event: crate::ports::KeyEvent,
) -> MenuAction {
    match event.code {
        KeyCode::Up => {
            selection.up();
            MenuAction::None
        }
        KeyCode::Down => {
            selection.down();
            MenuAction::None
        }
        KeyCode::Enter => items
            .get(selection.index())
            .map_or(MenuAction::None, |item| MenuAction::Selected(item.clone())),
        KeyCode::Esc => MenuAction::Cancelled,
        KeyCode::Char(c) => {
            // Hotkey jump: find the first item with `key == Some(c)`.
            if let Some(idx) = items.iter().position(|i| i.key == Some(c)) {
                selection.set(idx);
            }
            MenuAction::None
        }
        _ => MenuAction::None,
    }
}
