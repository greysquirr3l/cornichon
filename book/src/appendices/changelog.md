# Changelog

All notable changes to cornichon are documented here. The format is
based on [Keep a Changelog], and cornichon adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/

## [Unreleased]

### Changed

- **Dependencies** — updated the development dependency `criterion` from
  `0.5` to `0.8` and refreshed `Cargo.lock`. Criterion 0.8 deprecates
  `criterion::black_box` in favour of `std::hint::black_box`; the
  benchmark harness now imports the latter.
- **Lint profile** — the strict clippy profile moved from the `cargo l`
  alias in `.cargo/config.toml` to the `[workspace.lints]` table in the
  root `Cargo.toml`. Cargo appends user-supplied arguments *after* an
  alias body, so the previous `--`-separated alias turned any trailing
  argument (`-p cornichon`, `--workspace --all-targets`) into a rustc
  flag. With the profile in the manifest, `cargo l`, `cargo l -p
  cornichon`, and `cargo l --workspace --all-targets` all work.

### Fixed

- `Duration::from_secs(60)` in the spinner saturation test is now
  `Duration::from_mins(1)`, satisfying `clippy::duration_suboptimal_units`
  on Rust 1.96.0 (the toolchain CI pins).
- The `cargo l` alias no longer fails with `error: Unrecognized option`
  when given package-selection arguments.
- The `book` workflow's `environment` key moved from the workflow level to
  the `deploy` job. GitHub's workflow parser rejects a workflow-level
  `environment` (`Unexpected value 'environment'`), so the job was never
  created and every book run failed with zero jobs. The pinned mdbook is
  also updated from 0.4.40 to 0.5.4, and the install step now verifies
  the download's sha256 as its comment always claimed.

### Removed

- The redundant `package.homepage` metadata field, which Cargo reports as
  duplicating `package.repository`.

## [0.1.0] — 2026-08-31

The first public release.

> See the [Roadmap](./roadmap.md) for the 15 milestones closed in
> this release, grouped by area.

### Added

Grouped by area; full history of these items is in the
[Roadmap](./roadmap.md).

- **Menus**
  - `SlashMenu` — filterable `/command` palette with type-to-filter
    semantics, arrow + wrap navigation, hotkey support, and the Claude
    default command list (`/agents`, `/clear`, `/compact`, `/init`,
    `/model`, `/review`).
  - `SelectMenu` — single-select list with title, items, hotkey jumps,
    and a `query` / `filtered` / `filtered_indices` substring filter
    (hotkey priority over filter typing).
  - `PermissionPrompt` — Claude Code's "Do you want to proceed?"
    three-option chooser with pre-attached `1`/`2`/`3` hotkeys.
  - `ProjectPicker` — Grok's project-directory chooser with a
    trailing "custom" option that returns `MenuAction::Activated` so
    callers can switch to free-text input.
  - `ShortcutsModal` — Grok's Ctrl+x keyboard-shortcuts modal:
    categorized, expandable, two-stage Esc dismissal.
- **Interactivity**
  - `MenuDispatch` trait + `MenuEvent` enum unifying keyboard and mouse
    input; `dispatch(MenuEvent)` is the single entry point on every
    menu.
  - `show_scrollbar(&self) -> bool` and `show_scrollbar(self, on) -> Self`
    on each menu so callers can toggle the visible scrollbar without
    touching internal layout.
- **Spinners**
  - 61 `SpinnerStyle` variants across dots (braille), line, geometry,
    toggle, bouncing, pulse, noise, emoji, and special — names
    match [cli-spinners](https://github.com/sindresorhus/cli-spinners).
  - `Cadence` (ms-per-frame) with `FAST`/`NORMAL`/`SLOW` presets.
  - `Tick` (`Copy` monotonic counter).
  - `Spinner` state machine with `tick_at(Duration)` for clock-driven
    animation and `advance(Duration)` for delta-driven animation.
  - `TryFrom<&str>` / `TryFrom<String>` for spinner name → variant lookup.
  - `Spinner::with_frames(&[&str])` builder to override the default frame
    sequence for any style.
- **Forms**
  - `TextInput`, `Checkbox`, and `RadioGroup` in
    `cornichon::domain::forms`, each with a uniform `update()` /
    `apply()` API and a shared `apply_event` helper that adapts
    `MenuEvent` into per-widget key events. `Widget for &Type` impls in
    `adapters::ratatui::forms`.
- **Async** (opt-in, `--features async`)
  - `cornichon::ports::async_support::drive_until_action` — drive a
    menu's update loop from any async event source
    (`crossterm::event::EventStream`, `tokio::sync::mpsc::Receiver`,
    etc.) and resolve on the first non-`None` `MenuAction`.
  - No runtime dependency added; the feature only re-exports helpers.
- **Themes**
  - `CLAUDE`, `CODEX`, `GROK` consts from the brainless registry
    captures.
  - `Theme::default()` returns `CLAUDE`.
  - `Theme::from_env(var_name)` / `from_env_or(var_name, default)` /
    `lookup(name)` / `names()` / `all()` for runtime theme selection
    from an env var (e.g. `CORNICHON_THEME=grok`).
- **Other**
  - `cornichon::ports` — re-exports for `ratatui::Buffer`, `Rect`,
    `Style`, `Widget` and `crossterm::event::{KeyCode, KeyEvent,
KeyModifiers}` so consumers don't need a direct ratatui/crossterm
    dependency for the common types.
  - `cornichon::prelude` — one-stop `use` for everything you'd usually need.
- **Documentation**
  - Widget cookbook at `book/src/contributing/cookbook.md` — authoring
    a new menu end-to-end (domain item → state machine → `Widget for
&Menu` → render tests).
  - `book/src/contributing/stateful.md` — why cornichon deliberately
    diverges from `ratatui::StatefulWidget` (menus own their state and
    render via `frame.render_widget_ref(&menu, area)`).
  - `cargo doc --no-deps --document-private-items` is clean — every
    public item carries a `///` rustdoc block with at least one runnable
    example, and `use_self` / import-path lints pass.
- **Examples** (5 runnable bins)
  - `cornichon-slash` — slash-command palette demo
  - `cornichon-choice` — yes/no approval prompt
  - `cornichon-picker` — project picker with custom option
  - `cornichon-shortcuts` — categorized help modal
  - `cornichon-spinner` — 16 spinner styles animating concurrently

### Standards compliance

Per the project-level standards file at `AGENTS.md`:

- ✅ `src/{domain,ports,application,adapters}` internal layout
- ✅ `TryFrom<&str>` parsing at every boundary
- ✅ Total `update()` semantics; no `Result` returns on the update path
- ✅ Newtypes around `Copy` primitives are themselves `Copy`
- ✅ `const fn` where feasible (with `#[expect]` annotations for the
  Rust 1.98 not-yet-const-stable `Index`/`Ord`/`PartialEq` blockers)
- ✅ No `#[allow(clippy::…)]` suppressions anywhere
- ✅ No `.unwrap()` / `.expect()` / `.panic!()` — even in tests
- ✅ `indexing_slicing`-safe iteration throughout
- ✅ Public API surface (re-exports, prelude, doctest examples)
- ✅ Rust 1.96 / edition 2024
- ✅ Pure-domain state machines, no `Instant::now` inside the library

[Unreleased]: #unreleased
[0.1.0]: #010--2026-08-31
