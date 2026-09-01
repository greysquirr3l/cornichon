//! One module per menu type.
//!
//! Each module owns its `update` logic and its builder. The
//! [`crate::adapters::ratatui`] module registers a `WidgetRef` impl on each
//! type — the rendering side stays out of these files.

pub mod choice;
pub mod common;
pub mod picker;
pub mod select;
pub mod shortcuts;
pub mod slash;

pub use choice::{PermissionPrompt, PermissionPromptBuilder};
pub use picker::{ProjectPicker, ProjectPickerBuilder};
pub use select::{SelectMenu, SelectMenuBuilder};
pub use shortcuts::{ShortcutsGroup, ShortcutsItem, ShortcutsModal, ShortcutsModalBuilder};
pub use slash::{SlashMenu, SlashMenuBuilder};
