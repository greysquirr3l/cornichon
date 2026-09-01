# cornichon

[![CI](https://github.com/greysquirr3l/cornichon/actions/workflows/ci.yml/badge.svg)](https://github.com/greysquirr3l/cornichon/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/cornichon)](https://docs.rs/cornichon)
[![Crates.io](https://img.shields.io/crates/v/cornichon)](https://crates.io/crates/cornichon)
[![License: MIT](https://img.shields.io/crates/l/cornichon)](LICENSE)
[![Rust 1.96+](https://img.shields.io/badge/rust-1.96%2B-blue)](https://www.rust-lang.org)

> Composable Rust library for terminal-UI menu patterns — slash-command
> palettes, permission prompts, project pickers, keyboard-shortcut modals —
> themed after Claude Code, Codex, and Grok CLI captures.

cornichon packages **two surfaces** behind a single dependency:

| Surface      | Widgets / Styles                                                                               |
| ------------ | ---------------------------------------------------------------------------------------------- |
| **Menus**    | `SlashMenu`, `SelectMenu`, `PermissionPrompt`, `ProjectPicker`, `ShortcutsModal`               |
| **Spinners** | 61 deterministic animation styles across dots, line, geometry, toggle, bounce, pulse, emoji, … |

Both surfaces integrate with [`ratatui`] via the standard
`frame.render_widget(&menu_or_spinner, area)` call. Consumers don't need to
learn a new widget trait — cornichon implements `Widget` for `&Menu` and
`&Spinner`, so the existing ratatui render loop works unchanged.

The visual designs are taken from the [`brainless`] shadcn/ui registry
(the Claude / Codex / Grok UIs). Three themes ship with the crate as
[`CLAUDE`], [`CODEX`], and [`GROK`]; custom themes are plain struct literals.

```text
┌──────────────────────────────────────────┐
│ > /clear     Clear conversation history  │  ← SlashMenu (Claude theme)
│   /compact   Summarize the conversation  │
│   /init      Initialize a CLAUDE.md      │
└──────────────────────────────────────────┘
   ⠋ spinning… 5h ago          ← ProjectPicker (Grok theme)
```

[`brainless`]: https://brainless.swerdlow.dev
[`ratatui`]: https://ratatui.rs
[`CLAUDE`]: https://docs.rs/cornichon/latest/cornichon/constant.CLAUDE.html
[`CODEX`]: https://docs.rs/cornichon/latest/cornichon/constant.CODEX.html
[`GROK`]: https://docs.rs/cornichon/latest/cornichon/constant.GROK.html

---

## Quickstart

A working `SlashMenu` in ~30 lines of ratatui wiring, end-to-end.

```rust,no_run
use cornichon::prelude::*;
use cornichon::ports::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Widget};

fn main() -> Result<(), cornichon::Error> {
    let mut menu = SlashMenu::with_defaults()?;

    // In a real app, drive this from your event loop:
    let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    let action = menu.update(event);
    assert!(matches!(action, MenuAction::Selected(_) | MenuAction::None));
    Ok(())
}
```

The full event-loop wiring (terminal setup, raw mode, alt-screen, render) is
in the book — see **[Quickstart](book/book/quickstart.html)** for a complete
~50-line example. The only "cornichon" lines you'll ever write are
`with_defaults()` / `menu.update(key)`; everything else is stock ratatui +
crossterm. cornichon is a widget library, not a framework.

---

## Install

```toml
[dependencies]
cornichon = "0.1"
ratatui = "0.30"
crossterm = "0.29"
```

cornichon re-exports the `ratatui` / `crossterm` types it touches (`KeyEvent`,
`Buffer`, `Rect`, `Widget`) under [`cornichon::ports`] so you don't need a
separate `use ratatui::…` for the common cases — but if you'd rather name
ratatui directly in your own code, both work.

No feature flags today. Future additions (themed bundles, async runtime
support) will land as opt-in features rather than default-on behavior.

[`cornichon::ports`]: https://docs.rs/cornichon/latest/cornichon/ports/index.html

---

## Examples

Every widget ships with a runnable demo. From the repo root:

```bash
cargo run -p cornichon-slash       # SlashMenu
cargo run -p cornichon-choice      # PermissionPrompt
cargo run -p cornichon-picker      # ProjectPicker with trailing "custom"
cargo run -p cornichon-shortcuts   # ShortcutsModal
cargo run -p cornichon-spinner     # 16 spinner styles animating at once
```

Press <kbd>q</kbd> or <kbd>Esc</kbd> in any demo to quit.

---

## Design philosophy

> **Be a tiny, boring, well-typed piece of infrastructure that solves one thing well.**

1. **Pure-domain state machines, framework-agnostic rendering.** Every menu's
   `update()` returns a `MenuAction` enum. The state machine has zero
   knowledge of `ratatui`. Renderers live in `adapters/` and are replaceable.

2. **Total `update()` semantics.** Every key event returns a `MenuAction`.
   Invalid events are `MenuAction::None`, not `Err`. Menus can never enter
   an "invalid" state — they always have a valid cursor and always have
   something to draw.

3. **Parse, don't validate.** Raw `&str` input is funneled through
   `TryFrom<(&str, &str)>` on `MenuItem` / `AnnotatedItem` at construction
   time. After construction, those types are invariant-carrying — empty
   names, newlines-in-names, etc. are _unrepresentable_.

4. **Standards-compliant Rust.** No `.unwrap()` / `.expect()` / `panic!()` —
   even in tests. Newtypes wrapping `Copy` primitives are themselves `Copy`.
   `#[expect(...)]` over `#[allow(...)]`. `const fn` wherever feasible. The
   internal layout is the standard
   `src/{domain,ports,application,adapters}`.

5. **No runtime dependencies beyond ratatui + crossterm.** No async runtime,
   no `Instant::now()` inside the library, no background threads. Spinners
   are deterministic — the caller's event loop drives the ticks.

---

## Architecture

Four-layer hexagonal layout — domain is the innermost ring, free of
infrastructure; application holds the state machines; adapters wrap
external systems (here: ratatui).

```
crates/cornichon/src/
├── domain/         # pure value objects, enums, errors (no ratatui, no crossterm)
│   ├── item.rs     # MenuItem, AnnotatedItem  (TryFrom<(&str, &str)>)
│   ├── action.rs   # MenuAction
│   ├── theme.rs    # Theme + CLAUDE / CODEX / GROK consts
│   ├── error.rs    # thiserror Error enum
│   └── selection.rs # Selection (Copy cursor primitive)
├── application/    # state machines, use-case logic
│   └── menu/       # one module per widget: slash, select, choice, picker, shortcuts
├── ports/          # consumer-facing type re-exports (KeyEvent, Buffer, Rect, …)
└── adapters/
    └── ratatui/    # WidgetRef impls for each menu
```

Per the pragmatic trait-split decision, cornichon depends on `ratatui`
and `crossterm` directly. The "ports" are _type re-exports_ in
`src/ports/` — `crossterm::event::KeyEvent` is re-exported as
`cornichon::ports::KeyEvent`. Each menu implements
`ratatui::widgets::WidgetRef` so the standard `frame.render_widget_ref(&menu, area)`
call works without consumers naming ratatui types.

See **[Architecture](book/book/architecture.html)** for the full design notes.

---

## Documentation

- 📖 **[mdBook](book/book/introduction.html)** — tutorials, per-widget
  references, theme gallery, spinner style catalog (61 styles).
- 🦀 **[docs.rs/cornichon](https://docs.rs/cornichon)** — API reference.
- 📚 **[Examples](#examples)** — five runnable demos, one per widget
  plus the spinner showcase.

---

## Project

- **Maintainer:** greysquirr3l
- **Languages:** Rust (1.96 stable, edition 2024)
- **License:** MIT
- **CI:** `cargo l --workspace --all-targets` + `cargo test --workspace` +
  `cargo deny check` on every PR.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the widget cookbook
(how to add a new menu end-to-end across the four layers).
