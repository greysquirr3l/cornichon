//! Async-friendly helpers for menu event loops.
//!
//! This module is **feature-gated** behind the `async` Cargo feature:
//! ```toml
//! cornichon = { version = "0.1", features = ["async"] }
//! ```
//!
//! Without the `async` feature, use the synchronous `update` method
//! directly in your `tokio::select!` loop — no `await` is required for a
//! pure state-machine call.
//!
//! # Quickstart
//!
//! The [`drive_until_action`] helper loops an async event source,
//! forwards each [`MenuEvent`](crate::ports::MenuEvent) to the menu's
//! `update` method, and returns the first non-[`MenuAction::None`](
//! crate::ports::MenuAction::None) action. Stops if the source closes.
//!
//! ```ignore
//! use cornichon::prelude::*;
//! use cornichon::ports::async_support::drive_until_action;
//!
//! let action = drive_until_action(&mut menu, key_receiver).await;
//! ```
//!
//! # Why a free function, not a trait?
//!
//! Each menu type owns its `update` method directly. A trait would
//! require a `Menu` trait that conflicts with the existing inherent
//! `update` methods. A free generic function that takes an `update`
//! closure keeps this module orthogonal to the rest of the crate
//! while still being generic over any cornichon menu type.

use crate::MenuAction;
use crate::ports::MenuEvent;

/// Drive a menu's update loop from an async event source.
///
/// `next_event` is polled for each iteration via `next_event().await`;
/// it must yield the next event, or `None` if the source is closed.
///
/// The menu is mutated in place. Mouse events are passed through but
/// default menus return [`MenuAction::None`] for them — the consumer
/// reads the menu's `selection()` etc. afterwards if they need a
/// click position.
///
/// Returns the first `MenuAction::Selected` or `MenuAction::Cancelled`
/// produced by the menu, or `None` if the source closes first.
pub async fn drive_until_action<M, F, Fut>(menu: &mut M, mut next_event: F) -> Option<MenuAction>
where
    M: MenuDispatcher,
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<MenuEvent>>,
{
    while let Some(event) = next_event().await {
        let action = menu.dispatch(event);
        if !matches!(action, MenuAction::None) {
            return Some(action);
        }
    }
    None
}

/// A small adapter trait so `drive_until_action` can call `dispatch`
/// uniformly across all cornichon menu types.
///
/// Each menu implements this via its inherent `dispatch` method
/// (added in the mouse-support feature work). This trait is
/// **sealed** to the crate — only the menu types in `application::*`
/// implement it.
pub trait MenuDispatcher {
    /// Apply a [`MenuEvent`] and return the resulting action.
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction;
}

// Implement `MenuDispatcher` for every menu type. Each is just a
// one-liner that delegates to the inherent `dispatch` method.
//
// We list them by hand here rather than via a blanket impl to keep
// the trait surface minimal and self-documenting at the call site.

impl MenuDispatcher for crate::application::SelectMenu {
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl MenuDispatcher for crate::application::PermissionPrompt {
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl MenuDispatcher for crate::application::ProjectPicker {
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl MenuDispatcher for crate::application::SlashMenu {
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}

impl MenuDispatcher for crate::application::ShortcutsModal {
    fn dispatch(&mut self, event: MenuEvent) -> MenuAction {
        crate::ports::dispatch(self, event)
    }
}
