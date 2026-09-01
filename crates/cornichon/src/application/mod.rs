//! Application layer — state machines for each widget type.
//!
//! This is where the per-widget logic lives: selection tracking, key-event
//! dispatch, filter logic, expansion state, animation ticks. The
//! application layer imports from [`crate::domain`] (value objects) and
//! [`crate::ports`] (input/output type re-exports) but knows nothing about
//! rendering — that's the [`crate::adapters`] layer's job.
//!
//! # Layout
//!
//! - [`filter`]: the `filter_indices` helper used by [`menu::SlashMenu`].
//! - [`menu`]: one module per *menu* widget type (`SelectMenu`,
//!   `PermissionPrompt`, `ProjectPicker`, `SlashMenu`, `ShortcutsModal`).
//! - [`spinner`]: the [`Spinner`](spinner::Spinner) state machine.
//!   Sibling to `menu` because it isn't key-driven — it's clock-driven.

pub mod filter;
pub mod forms;
pub mod menu;
pub mod spinner;

pub use filter::filter_indices;
pub use menu::{
    PermissionPrompt, PermissionPromptBuilder, ProjectPicker, ProjectPickerBuilder, SelectMenu,
    SelectMenuBuilder, ShortcutsGroup, ShortcutsItem, ShortcutsModal, ShortcutsModalBuilder,
    SlashMenu, SlashMenuBuilder,
};
pub use spinner::{Spinner, SpinnerBuilder};
