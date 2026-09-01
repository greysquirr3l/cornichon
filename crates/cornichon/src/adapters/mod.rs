//! Adapters layer — infrastructure-facing implementations.
//!
//! Per the `src/{domain,ports,application,adapters}` standard, this is the
//! outermost ring: it depends on everything else and on external libraries
//! (currently just `ratatui`).
//!
//! The [`ratatui`] submodule registers `Widget` impls on `&Menu` for each
//! menu type, so consumers can call `frame.render_widget(&menu, area)`
//! without naming ratatui types.

pub mod ratatui;
