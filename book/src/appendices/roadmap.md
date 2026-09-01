# Roadmap

The 0.1.0 release closed 15 milestones that took the crate from
"five menus on a ratatui buffer" to a fully published, linted,
benchmarked, documented, and CI-validated library. This page is
the historical record of those items, grouped by area.

> All 15 items below shipped in [0.1.0](./changelog.md#010--2026-08-31).
> The full changelog entry for the release is in the
> [Changelog](./changelog.md).

---

## Build & release

1. **CI workflow** — [`.github/workflows/ci.yml`](https://github.com/greysquirr3l/cornichon/blob/main/.github/workflows/ci.yml)
   runs the strict `nick.md` clippy profile
   (`-W clippy::{all,pedantic,nursery,cargo,perf}` plus 6 denied
   lints and `-D warnings`) on every PR, plus `cargo test`,
   `cargo fmt --check`, and `cargo deny check`.

2. **`CONTRIBUTING.md`** — author guide covering widget design
   rules, the `domain / ports / application / adapters` layout, the
   total-`update` semantics, and the lint expectations enforced by
   CI.

3. **mdbook → GitHub Pages deploy** —
   [`.github/workflows/book.yml`](https://github.com/greysquirr3l/cornichon/blob/main/.github/workflows/book.yml)
   builds the book with `mdbook build` and publishes to Pages on
   every push to `main`.

---

## Documentation

4. **Widget cookbook** —
   [`book/src/contributing/cookbook.md`](https://github.com/greysquirr3l/cornichon/blob/main/book/src/contributing/cookbook.md)
   walks through authoring a new menu end-to-end: defining the
   domain item, wiring the state machine, implementing
   `Widget for &Menu`, and adding render tests.

5. **`cargo doc` cleanup** — every public item carries a `///`
   rustdoc block with at least one runnable example. Crate-level
   `use_self`, `clippy::use_self`, and import-path lints are clean
   under `cargo doc --no-deps --document-private-items`.

6. **Why no `StatefulWidget?`** —
   [`book/src/contributing/stateful.md`](https://github.com/greysquirr3l/cornichon/blob/main/book/src/contributing/stateful.md)
   documents the deliberate divergence from
   `ratatui::StatefulWidget`: cornichon's menus own their state and
   render via `frame.render_widget_ref(&menu, area)`, not the
   `&mut State<Menu>` split.

---

## Interactivity

7. **Mouse support** — `MenuDispatch` trait + `MenuEvent` enum
   (`Key` / `Mouse`) so a single `dispatch(MenuEvent)` call handles
   keyboard and mouse uniformly across all five menus.

8. **Scrollbar integration** — each menu exposes
   `show_scrollbar(&self) -> bool` and `show_scrollbar(self, on) ->
Self`, so callers can toggle the visible scrollbar without
   touching internal layout.

9. **`SelectMenu` filter** — `query`, `filtered`, and
   `filtered_indices` fields, with **hotkey-priority over filter
   typing**: the menu scans `item.key == Some(c)` first, before
   appending the char to the query.

---

## Spinners

10. **`Spinner::with_frames`** — lets callers override the default
    frame sequence for any style with a custom `&[&str]` slice via
    `Spinner::builder().frames(&[…]).build()`.

---

## Testing

11. **Snapshot tests** —
    [`crates/cornichon/tests/render.rs`](https://github.com/greysquirr3l/cornichon/blob/main/crates/cornichon/tests/render.rs)
    renders each menu into a fresh `Buffer` and asserts that
    specific cells contain the expected glyphs (the _manual golden_
    approach — no brittle stored-string snapshots).

12. **Criterion benches** —
    [`crates/cornichon/benches/state_machines.rs`](https://github.com/greysquirr3l/cornichon/blob/main/crates/cornichon/benches/state_machines.rs)
    measures filter, navigation, and update-loop throughput with a
    `make_item!` macro for safe construction. `html_reports` feature
    enabled so `cargo bench` produces a browsable report.

---

## Forms

13. **Form widgets** — `TextInput`, `Checkbox`, and `RadioGroup` in
    `cornichon::domain::forms`, each with a uniform `update()` /
    `apply()` API and a shared `apply_event` helper that adapts
    `MenuEvent` into per-widget key events.

---

## Async

14. **Async support** — feature-gated
    `cornichon::ports::async_support` re-exports for building menus
    on top of async input streams (`crossterm::event::EventStream`,
    `tokio::sync::mpsc::Receiver`, etc.). Enabled with
    `--features async`.

---

## Theming

15. **`Theme::from_env`** — `from_env`, `from_env_or`, `lookup`,
    `names`, and `all` constructors let apps pick a theme at runtime
    from an env var (e.g. `CORNICHON_THEME=grok`) instead of
    hard-coding one of the `CLAUDE` / `CODEX` / `GROK` consts.

---

**Total: 15 items / 0 deferred / 0 reverted.**
