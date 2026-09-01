//! Re-exports of `ratatui` rendering primitives used by the adapter layer.
//!
//! Consumers typically don't import from this module directly — the
//! adapters register `Widget` impls on `&Menu` so the standard
//! `frame.render_widget(&menu, area)` call works without naming ratatui
//! types. This module exists so that consumers who want to render to a
//! raw `Buffer` (for tests, custom backends, or non-frame contexts) can
//! without taking a direct ratatui dependency.
//!
//! # Note on `Widget` vs `WidgetRef`
//!
//! ratatui 0.30 has both [`Widget`] (stable, takes `self` by value) and
//! [`WidgetRef`](ratatui::widgets::WidgetRef) (gated behind the
//! `unstable-widget-ref` feature). cornichon ships menus that own their
//! state, so we implement `Widget for &Menu` — the stable pattern that
//! lets consumers render without depending on unstable ratatui features.

pub use ratatui::buffer::Buffer;
pub use ratatui::layout::Rect;
pub use ratatui::style::Style;
pub use ratatui::widgets::Widget;

/// Convenience alias for the receiver of a render call.
///
/// Currently `&mut Buffer`, matching ratatui's `Widget::render` signature.
/// Lives as an alias so adapters can be written against `RenderTarget`
/// rather than `Buffer` directly, leaving room to broaden (e.g. to support
/// double-buffered or diff-based renderers) without breaking consumers.
pub type RenderTarget<'a> = &'a mut Buffer;
