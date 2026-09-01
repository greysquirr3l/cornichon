# Changelog

All notable changes to cornichon are documented here. The format is
based on [Keep a Changelog], and cornichon adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/

## [Unreleased]

No changes yet.

## [0.1.0] — 2026-08-31

The first public release.

> See the [Roadmap](./roadmap.md) for the 15 milestones closed in
> this release, grouped by area.

### Added

- **Menus**
  - `SlashMenu` — filterable `/command` palette with type-to-filter
    semantics, arrow + wrap navigation, hotkey support, and the Claude
    default command list (`/agents`, `/clear`, `/compact`, `/init`,
    `/model`, `/review`).
  - `SelectMenu` — basic single-select list with title, items, and
    hotkey jumps.
  - `PermissionPrompt` — Claude Code's "Do you want to proceed?"
    three-option chooser with pre-attached `1`/`2`/`3` hotkeys.
  - `ProjectPicker` — Grok's project-directory chooser with a
    trailing "custom" option that returns `MenuAction::Activated` so
    callers can switch to free-text input.
  - `ShortcutsModal` — Grok's Ctrl+x keyboard-shortcuts modal:
    categorized, expandable, two-stage Esc dismissal.
- **Spinners**
  - 61 `SpinnerStyle` variants across dots (braille), line, geometry,
    toggle, bouncing, pulse, noise, emoji, and special — names
    match [cli-spinners](https://github.com/sindresorhus/cli-spinners).
  - `Cadence` (ms-per-frame) with `FAST`/`NORMAL`/`SLOW` presets.
  - `Tick` (`Copy` monotonic counter).
  - `Spinner` state machine with `tick_at(Duration)` for clock-driven
    animation and `advance(Duration)` for delta-driven animation.
  - `TryFrom<&str>` / `TryFrom<String>` for spinner name → variant lookup.
- **Themes**
  - `CLAUDE`, `CODEX`, `GROK` consts from the brainless registry
    captures.
  - `Theme::default()` returns `CLAUDE`.
- **Other**
  - `cornichon::ports` — re-exports for `ratatui::Buffer`, `Rect`,
    `Style`, `Widget` and `crossterm::event::{KeyCode, KeyEvent,
KeyModifiers}` so consumers don't need a direct ratatui/crossterm
    dependency for the common types.
  - `cornichon::prelude` — one-stop `use` for everything you'd usually need.
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
