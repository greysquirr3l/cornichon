//! cornichon — composable TUI menu patterns themed after Claude Code, Codex, and Grok.
//!
//! cornichon is a Rust library that ports the menu patterns used by the
//! major AI coding CLIs to [`ratatui`]. The five widget types
//! ([`SlashMenu`], [`SelectMenu`], [`PermissionPrompt`], [`ProjectPicker`],
//! [`ShortcutsModal`]) ship as builders and render via the standard
//! `frame.render_widget_ref(&menu, area)` call — no ratatui knowledge
//! required at the consumer site beyond passing the area.
//!
//! The visual designs are taken from the [`brainless`] shadcn registry
//! (Claude / Codex / Grok UIs). Three themes ship with the crate as
//! [`CLAUDE`], [`CODEX`], and [`GROK`]; custom themes are struct literals.
//!
//! [`ratatui`]: https://ratatui.rs
//! [`brainless`]: https://brainless.swerdlow.dev
//!
//! # Quickstart
//!
//! ```rust,no_run
//! use cornichon::prelude::*;
//! use cornichon::ports::{KeyCode, KeyEvent, KeyModifiers};
//!
//! let mut menu = SlashMenu::with_defaults()?;
//! // ... drive menu.update(KeyEvent) in your event loop ...
//! # Ok::<(), cornichon::Error>(())
//! ```
//!
//! # Architecture
//!
//! Internal layout follows the project's
//! [`src/{domain,ports,application,adapters}`] standard
//! (see [`AGENTS.md`](https://github.com/greysquirr3l/cornichon/blob/main/AGENTS.md) for the full discussion):
//!
//! - [`domain`]: pure value objects ([`MenuItem`], [`Theme`], [`Selection`],
//!   [`MenuAction`], [`Error`]). No `ratatui`, no `crossterm`.
//! - [`ports`]: type re-exports so consumers don't need a direct
//!   `ratatui` / `crossterm` dependency to type-annotate handlers.
//! - [`application`]: per-menu state machines (`update` → [`MenuAction`]).
//!   Total: invalid events are [`MenuAction::None`], not `Result::Err`.
//! - [`adapters`]: ratatui `WidgetRef` impls for each menu type.
//!
//! # Conventions
//!
//! - `update` is **total**: every event produces a [`MenuAction`]. There
//!   is no `Result` return path because menus cannot be in an invalid
//!   state.
//! - Builders consume `self` and return `Self` (true builder pattern).
//! - The only place `Result` appears in the public API is at the
//!   construction boundary (`build()`), where parsed input is funneled
//!   through [`TryFrom`].
//!
//! [`src/{domain,ports,application,adapters}`]: ./index.html#architecture

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub use application::{
    PermissionPrompt, PermissionPromptBuilder, ProjectPicker, ProjectPickerBuilder, SelectMenu,
    SelectMenuBuilder, ShortcutsGroup, ShortcutsItem, ShortcutsModal, ShortcutsModalBuilder,
    SlashMenu, SlashMenuBuilder, Spinner, SpinnerBuilder, filter_indices,
    forms::{Changed, FormEvent},
};
pub use domain::{
    CLAUDE, CODEX, Cadence, Error, GROK, MenuAction, Result, Selection, SpinnerStyle, Theme, Tick,
    UnknownSpinnerError,
    forms::{Checkbox, RadioGroup, TextInput},
    item::{AnnotatedItem, MenuItem},
};
pub use ports::{MenuEvent, dispatch};

/// Convenient re-exports for the most common cornichon types.
///
/// ```rust,ignore
/// use cornichon::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        application::{
            PermissionPrompt, PermissionPromptBuilder, ProjectPicker, ProjectPickerBuilder,
            SelectMenu, SelectMenuBuilder, ShortcutsGroup, ShortcutsItem, ShortcutsModal,
            ShortcutsModalBuilder, SlashMenu, SlashMenuBuilder, Spinner, SpinnerBuilder,
            filter_indices,
            forms::{Changed, FormEvent},
        },
        domain::{
            CLAUDE, CODEX, Cadence, Error, GROK, MenuAction, Result, Selection, SpinnerStyle,
            Theme, Tick,
            forms::{Checkbox, RadioGroup, TextInput},
            item::{AnnotatedItem, MenuItem},
        },
        ports::{
            Buffer, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind, Rect,
            Style, Widget,
        },
    };
}
