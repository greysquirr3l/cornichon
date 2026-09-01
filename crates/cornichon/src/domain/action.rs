//! The action enum that every menu's `update` returns.
//!
//! Per the consumer-owned-traits standard, menus expose a single narrow port
//! (`update -> MenuAction`) that the caller matches on. There is intentionally
//! no `Result` here — menus are total. "Nothing happened" is represented by
//! [`MenuAction::None`], not by an `Err`.

/// What a menu did in response to a key event.
///
/// Constructed by [`crate::application::menu::*::update`] methods and matched
/// on by the caller. The variants are deliberately flat — no nested state —
/// because callers should be able to handle each case with a single arm.
///
/// # Examples
///
/// ```rust
/// use cornichon::domain::MenuAction;
///
/// fn handle(action: MenuAction) {
///     match action {
///         MenuAction::None => {}
///         MenuAction::Selected(item) => println!("chose: {}", item.name),
///         MenuAction::Cancelled => println!("aborted"),
///         MenuAction::Activated => println!("activated (no selection)"),
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    /// The event was not meaningful for this menu; nothing changed.
    ///
    /// Most events fall here — typing a non-control character in a
    /// [`SelectMenu`](crate::application::menu::select::SelectMenu), for example.
    None,

    /// The user selected a menu item.
    ///
    /// Carries the *full* item (clone is cheap — `MenuItem` is a handful of
    /// small `String`s — and per counterintuitive-patterns #1 "Sometimes You
    /// Should Just Clone", the alternative is a lifetime parameter that
    /// infects every caller).
    Selected(crate::domain::item::MenuItem),

    /// The user dismissed the menu without selecting (Esc, etc.).
    Cancelled,

    /// The user activated an element that doesn't correspond to an item.
    ///
    /// Used by [`PickerMenu`](crate::application::menu::picker::PickerMenu) when
    /// the "custom" trailing option is chosen, and by
    /// [`ShortcutsModal`](crate::application::menu::shortcuts::ShortcutsModal)
    /// when its close button is activated.
    Activated,
}
