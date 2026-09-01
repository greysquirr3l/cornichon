# Testing

cornichon's widgets are designed to be **trivial to unit-test**:

- Pure data + `update(&mut self, KeyEvent) -> MenuAction` — no threads,
  no async, no `Instant::now()`, no filesystem, no I/O.
- Renderers take a `&mut Buffer` — also pure data, no I/O.
- `Default::default()` gives you a working menu in one line.

Every test in the crate's test suite is a state-machine unit test
(`cargo test -p cornichon`) that follows the patterns below. Render
tests live in the adapter layer; this page documents the patterns.

## State-machine unit tests

The canonical shape for a menu test:

```rust,no_run
use cornichon::prelude::*;

#[test]
fn arrow_down_advances_cursor() -> Result<(), cornichon::domain::Error> {
    let mut menu = SelectMenu::builder()
        .items([
            MenuItem::try_from(("a", "first"))?,
            MenuItem::try_from(("b", "second"))?,
            MenuItem::try_from(("c", "third"))?,
        ])
        .build()?;

    assert_eq!(menu.selection(), 0);
    let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(menu.selection(), 1);

    Ok(())
}
```

A few conventions, all enforced by the project's lint profile:

- `?` propagation, not `.unwrap()` — the function returns
  `Result<(), cornichon::domain::Error>`. Tests that exercise fallible
  builder APIs must propagate.
- `assert!` / `assert_eq!` / `assert_ne!` macros are fine. `panic!` is
  not — it's a `clippy::panic` deny lint.
- All key events are constructed inline via
  `KeyEvent::new(KeyCode::…, KeyModifiers::NONE)`. No fixtures.
- Don't drive the render path — see [`Buffer render tests`](#buffer-render-tests)
  below.

### Testing the `Cancelled` path

```rust,no_run
#[test]
fn esc_cancels() -> Result<(), cornichon::domain::Error> {
    let mut menu = PermissionPrompt::with_defaults()?;
    let action = menu.update(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(action, MenuAction::Cancelled);
    Ok(())
}
```

`MenuAction` is `Eq + PartialEq`, so direct equality is fine.

### Testing custom-selection dispatch

`ProjectPicker` returns `MenuAction::Activated` for the trailing
"custom" option, `MenuAction::Selected` for everything else:

```rust,no_run
#[test]
fn custom_option_returns_activated() -> Result<(), cornichon::domain::Error> {
    let mut picker = ProjectPicker::with_defaults()?;
    let _ = picker.update(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    assert!(picker.is_custom_selected());
    let action = picker.update(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(action, MenuAction::Activated);
    Ok(())
}
```

### Testing filter behavior

`SlashMenu` re-runs `filter_indices` on any printable char:

```rust,no_run
#[test]
fn typing_filters_case_insensitively() -> Result<(), cornichon::domain::Error> {
    let mut menu = SlashMenu::with_defaults()?;
    let _ = menu.update(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    let _ = menu.update(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
    assert_eq!(menu.query(), "AG");
    assert_eq!(menu.filtered_indices().len(), 1);
    Ok(())
}
```

### Testing wrap-around

```rust,no_run
#[test]
fn down_wraps_at_bottom() -> Result<(), cornichon::domain::Error> {
    let mut menu = SelectMenu::builder()
        .items([
            MenuItem::try_from(("a", "1"))?,
            MenuItem::try_from(("b", "2"))?,
        ])
        .build()?;
    let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(menu.selection(), 0); // wraps
    Ok(())
}
```

## Buffer render tests

The ratatui adapter renders into a [`Buffer`] which is a 2D grid of
`Cell`s. You can construct a `Buffer` of any size without a terminal:

```rust,no_run
use cornichon::prelude::*;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[test]
fn renders_first_frame_of_dots_into_buffer() {
    let spinner = Spinner::builder()
        .style(SpinnerStyle::Dots)
        .build();
    let area = Rect::new(0, 0, 20, 1);
    let mut buf = Buffer::empty(area);
    (&spinner).render_ref(area, &mut buf);  // uses WidgetRef
    // or, for the stable Widget API:
    // (&spinner).render(area, &mut buf);

    // Frame 0 of Dots is "⠋" (U+280B). Check the first cell.
    let cell = &buf[(0, 0)];
    assert_eq!(cell.symbol(), "⠋");
}
```

> **Note:** ratatui 0.30's stable `Widget` trait takes `self` by value.
> cornichon implements `Widget for &Spinner`, so you call
> `(&spinner).render(area, &mut buf)`. (`WidgetRef` would let you call
> `spinner.render_ref(...)` directly, but it's gated behind the
> `unstable-widget-ref` feature.)

[`Buffer`]: https://docs.rs/ratatui/latest/ratatui/buffer/struct.Buffer.html

### Snapshot-style golden tests

For regression coverage, snapshot the buffer's `String()` and compare
to a checked-in string. (A more sophisticated golden harness exists in
[ratatuui's tests] using `insta`; if you're already using `insta` in
your project, that works fine here too.)

[ratatuui's tests]: https://github.com/ratatui/ratatui/tree/main/ratatui/tests

## Doctest pattern

Public rustdoc examples should be runnable as `cargo test --doc`.
cornichon follows the standards in `AGENTS.md`:

````rust
//! ```rust
//! use cornichon::prelude::*;
//!
//! let mut s = SlashMenu::with_defaults()?;
//! let _ = s.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
//! # Ok::<(), cornichon::Error>(())
//! ```
//!
//! (`?` propagation requires the doctest function to return `Result`,
//! which is what the trailing `# Ok::<…>(())` does.)
````

If you write a doctest that does anything more elaborate (calls `unsafe`,
constructs long-lived references, spawns threads), mark it with
ignore:

````rust
//! ```rust,ignore
//! let mut last = std::time::Instant::now();
//! loop { /* ... */ }
//! ```
````

## Quick-reference: lint profile

The strict profile is encoded in `.cargo/config.toml` (alias `cargo l`).
For a single check, the equivalent is:

```bash
RUSTC_WRAPPER= SCCACHE_DISABLE=1 cargo clippy -p cornichon --all-targets -- \
  -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::cargo -W clippy::perf \
  -A clippy::module_name_repetitions -A clippy::must_use_candidate \
  -A clippy::missing_errors_doc -A clippy::missing_panics_doc \
  -A clippy::struct_excessive_bools -A clippy::multiple_crate_versions \
  -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic \
  -D clippy::indexing_slicing -D clippy::cast_ptr_alignment \
  -D clippy::suspicious -D warnings
```

If `cargo l` is green, your tests will merge.
