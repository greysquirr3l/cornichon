# Contributing · Widget cookbook

Step-by-step recipe for adding a new widget to cornichon. By the end
you'll have a 6-row picker with a custom [`Widget for &Menu`] impl,
tested, themed, and rendered. Eight steps; about 200 lines of code.

> The standards file (`AGENTS.md` at the workspace root) is the
> authoritative source — this cookbook assumes you've read it.

## 0. Pick a widget to build

Let's build **ColorPicker** — a list of named ANSI colors with live
preview:

```text
┃ pick a color
┃
┃   1 > red       ████
┃     2 green     ████
┃     3 blue      ████
┃     4 cyan      ████
┃     5 magenta   ████
┃     6 default   ████
```

Each row is a number, a name, and a 4-cell preview swatch. Arrow keys
move; Enter confirms; Esc cancels.

## 1. Domain type

`crates/cornichon/src/domain/picker_demo.rs`:

```rust
use crate::domain::{Error, Result};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedColor {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Default,
}

impl NamedColor {
    pub const fn name(self) -> &'static str { /* ... */ }
}

impl fmt::Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// Boundary type — a TryFrom at the parsing edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorRow {
    pub index: u8,
    pub color: NamedColor,
}

impl TryFrom<&str> for ColorRow {
    type Error = Error;
    fn try_from(_s: &str) -> Result<Self> {
        Err(Error::EmptyMenu) // placeholder; replace with real parse
    }
}
```

Add `pub mod picker_demo;` to `crates/cornichon/src/domain/mod.rs`.

### Why this layer matters

The `domain` layer is pure value objects. **No** `ratatui`,
**no** `crossterm`. Boundary types (like `ColorRow` once it accepts raw
input) implement `TryFrom<...>` so the rest of the widget never has to
defend against malformed strings. The compiler enforces the invariant.

## 2. Application state machine

`crates/cornichon/src/application/menu/color_picker.rs`:

```rust
use crate::domain::{NamedColor, Result};
use crate::domain::error::Error;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    rows: Vec<NamedColor>,
    selection: usize,
}

impl ColorPicker {
    pub fn builder() -> ColorPickerBuilder { ColorPickerBuilder::default() }

    pub fn items<I>(mut self, rows: I) -> Self
    where I: IntoIterator<Item = NamedColor> { /* ... */ }

    /// Total. Returns MenuAction — never Result.
    pub fn update(&mut self, _event: KeyEvent) -> MenuAction {
        // Demo: just Enter selects at row 0; no navigation in this
        // minimal example. Real impl follows the SlashMenu pattern.
        MenuAction::None
    }
}

#[derive(Debug, Default, Clone)]
pub struct ColorPickerBuilder { rows: Vec<NamedColor> }

impl ColorPickerBuilder {
    pub fn items<I>(mut self, rows: I) -> Self
    where I: IntoIterator<Item = NamedColor> {
        self.rows = rows.into_iter().collect();
        self
    }
    pub fn build(self) -> Result<ColorPicker> {
        if self.rows.is_empty() {
            return Err(Error::EmptyMenu);
        }
        Ok(ColorPicker { rows: self.rows, selection: 0 })
    }
}
```

Add `pub mod color_picker;` to `crates/cornichon/src/application/menu/mod.rs`.

### Update semantics

`update` is **total** — it never panics, never returns `Result`, and
treats unrecognized events as `MenuAction::None`. See `AGENTS.md` for
the rationale.

## 3. Ratatui adapter

`crates/cornichon/src/adapters/ratatui/color_picker.rs`:

```rust
use crate::application::ColorPicker;
use crate::ports::{Buffer, Rect, Widget};

impl Widget for &ColorPicker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 { return; }
        for (i, row) in self.rows().iter().enumerate() {
            // ...layout the row, set bg cells to the color, etc.
            // Left as an exercise; the pattern matches
            // `adapters/ratatui/select.rs`.
        }
    }
}
```

Add `pub mod color_picker;` to `crates/cornichon/src/adapters/ratatui/mod.rs`.

### Why `&ColorPicker`

`Widget for &ColorPicker` lets a single menu instance be passed as a
reference, mirroring how ratatui's own widgets compose. If a caller has
`ColorPicker` by value, `(&*self).render(...)` works transparently
through Deref.

## 4. Re-exports

Three files:

- `crates/cornichon/src/domain/mod.rs` — `pub mod picker_demo;`
- `crates/cornichon/src/application/mod.rs` — `pub use menu::color_picker::ColorPicker;`
- `crates/cornichon/src/lib.rs` — `pub use application::ColorPicker;`

If the widget is in `prelude`, add it there too.

## 5. Tests

**Unit test (state machine):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_selects_first_row() {
        let mut p = ColorPicker::builder()
            .items([NamedColor::Red, NamedColor::Green])
            .build()
            .unwrap();
        let action = p.update(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(matches!(action, MenuAction::Selected(ref c) if c.is_some()));
    }
}
```

**Doctest** (in the rustdoc for `ColorPicker`):

````rust
//! ```rust
//! use cornichon::prelude::*;
//!
//! let mut p = ColorPicker::builder()
//!     .items([NamedColor::Red, NamedColor::Blue])
//!     .build()?;
//! assert_eq!(p.items().len(), 2);
//! # Ok::<(), cornichon::Error>(())
//! ```
````

**Render test** (under `crates/cornichon/tests/render.rs`):

```rust
#[test]
fn color_picker_renders_two_rows() {
    let p = ColorPicker::builder()
        .items([NamedColor::Red, NamedColor::Blue])
        .build()
        .unwrap();
    let area = Rect::new(0, 0, 20, 3);
    let mut buf = Buffer::empty(area);
    (&p).render(area, &mut buf);
    // Assert the buffer contains "red" at (2, 0) and "blue" at (2, 1).
}
```

## 6. Theme

Themes flow through automatically — your renderer reads `self.theme()`
and uses `fg`, `fg_dim`, `fg_muted`, `accent`, and `border` like every
other widget. No special-casing needed.

## 7. Book chapter

`book/src/widgets/color-picker.md`:

```md
# ColorPicker

A list of named ANSI colors with a 4-cell preview swatch per row.

\`\`\`rust
use cornichon::prelude::\*;
let mut p = ColorPicker::builder()
.items([NamedColor::Red, NamedColor::Blue])
.build()?;
\`\`\`
```

Add the link to `book/src/SUMMARY.md` under the `Widgets` section.

## 8. Verify

```bash
cd /Users/ncampbell/Projects/rust/cornichon
cargo l --workspace --all-targets   # strict lint profile
cargo test --workspace               # unit + integration + doctest
mdbook build book                    # book chapter renders cleanly
```

All four must exit 0 before opening the PR.

[`Widget for &Menu`]: https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html
