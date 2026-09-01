# Architecture

cornichon's internal layout follows the standard
`src/{domain,ports,application,adapters}` pattern (see `AGENTS.md` at
the repo root for the full project-local standards file).

```text
crates/cornichon/src/
├── lib.rs                       # Re-exports + prelude
│
├── domain/                      # Inner ring — pure value objects, no I/O
│   ├── mod.rs
│   ├── item.rs                  # MenuItem, AnnotatedItem (TryFrom at boundary)
│   ├── action.rs                # MenuAction enum (what happened)
│   ├── selection.rs             # Selection — Copy cursor primitive
│   ├── theme.rs                 # Theme + CLAUDE/CODEX/GROK consts
│   ├── spinner.rs               # SpinnerStyle, Cadence, Tick
│   └── error.rs                 # thiserror Error enum
│
├── ports/                       # Type re-exports so consumers don't need
│   ├── input.rs                 # crossterm KeyEvent/KeyCode/KeyModifiers
│   └── render.rs                # ratatui Buffer/Rect/Style/Widget
│
├── application/                 # Outer ring — per-widget state machines
│   ├── mod.rs
│   ├── filter.rs                # filter_indices() helper used by SlashMenu
│   └── menu/
│       ├── common.rs            # handle_common_key shared across menus
│       ├── select.rs            # SelectMenu + builder
│       ├── choice.rs            # PermissionPrompt + builder
│       ├── picker.rs            # ProjectPicker + builder
│       ├── slash.rs             # SlashMenu + builder
│       ├── shortcuts.rs         # ShortcutsModal + builder + group types
│       └── spinner.rs           # Spinner + builder
│
└── adapters/                    # Outermost ring — external-system bindings
    └── ratatui/
        ├── common.rs            # render_row(), render_title()
        ├── select.rs            # impl Widget for &SelectMenu
        ├── choice.rs            # impl Widget for &PermissionPrompt
        ├── picker.rs            # impl Widget for &ProjectPicker
        ├── slash.rs             # impl Widget for &SlashMenu
        ├── shortcuts.rs        # impl Widget for &ShortcutsModal
        └── spinner.rs           # impl Widget for &Spinner
```

## Layered layout

The four rings are explicit dependencies, not "soft architecture":

```text
adapters/  →  application/  →  domain/
                                  ↘  ports/
```

- **`adapters/`** depends on `application/`, `domain/`, `ports/`,
  _and_ external libraries (`ratatui`). Anything infrastructure-touching
  lives here.
- **`application/`** depends on `domain/`. State machines, business
  logic, builders — no infrastructure.
- **`ports/`** re-exports types from `ratatui` / `crossterm`. It's
  tiny on purpose: pure re-exports and aliases.
- **`domain/`** depends on `core` (and through `ports/`, technically on
  `ratatui` for `Color`). Pure value objects, builders' inputs, errors.

The dependency direction is enforced by the workspace lint profile
(`cargo l -p cornichon --all-targets`) which denies `unwrap_used`,
`panic`, `indexing_slicing`, etc. — so a domain function that
accidentally imports from `application/` or `adapters/` still
compiles, but anything that smells like infrastructure gets caught.

## Pragmatic on ratatui

The internal note in `AGENTS.md` says it best:

> **Pragmatic on the renderer.** cornichon depends on `ratatui` and
> `crossterm` directly. There's no consumer-owned `Renderer` trait —
> the "ports" are _type re-exports_, not interfaces. Each menu
> implements `Widget for &Menu` so `frame.render_widget(&menu, area)`
> is the entire consumer-facing render API.

The reasoning: `ratatui` is the de-facto standard for Rust TUIs. A
consumer-owned render trait (a `Render trait` with `Buffer` / `Rect`
methods) would let consumers swap renderers, but in practice:

- 99% of consumers will use `ratatui`.
- The few that want something else (a wasm canvas, a tape renderer,
  a snapshot tester) can implement `Widget` themselves against cornichon's
  menus. The `Widget` trait is `&self`-ish, ergonomic, and trivial to
  re-implement.

The `ports::render` module exposes `Buffer`, `Rect`, `Style`, and
`Widget` so consumers don't have to add a direct `ratatui` dependency
just for the common types — but they can if they want richer
functionality (`Paragraph`, `Layout`, etc.).

The same pragmatic stance applies to input: cornichon consumes
`crossterm::event::KeyEvent`. If you want `termion` or `termwiz`, convert
events to `crossterm::event::KeyEvent` first.

## Total `update` semantics

Every menu's `update(&mut self, KeyEvent) -> MenuAction` is **total**:

- It returns `MenuAction` for every input. There is no `Result` return path.
- An invalid event is `MenuAction::None`, **not** `Err(InvalidEvent)`.
- Menus can never enter an "invalid" state — there's always a valid
  cursor, even in an empty list.
- Wrapping is the default (Up from row 0 lands on the last row).
- Hotkey mismatches are silent (no error beep).

This is the same contract [`ratatui::widgets::ListState::select`] exposes
and the one every TUIs-with-state library eventually converges on
(`bubbletea`, `Ink`, etc.).

[`ratatui::widgets::ListState::select`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.ListState.html#method.select

## Parse, don't validate

Raw `&str`/`String` input is funneled through `TryFrom` on
[`MenuItem`](https://docs.rs/cornichon/latest/cornichon/struct.MenuItem.html)
or [`AnnotatedItem`](https://docs.rs/cornichon/latest/cornichon/struct.AnnotatedItem.html)
at construction time. After construction, the types are **invariant-carrying**:

```rust
// TryFrom<(String, String)> for MenuItem
let empty_name: MenuItem = "".try_into().into();  // panics at compile time: ❌
// No way to construct a MenuItem whose `.name` is "" or contains a \n.
```

This means the rest of the library can be fearless about input: a
`MenuItem`'s `.name()` is guaranteed to be non-empty and
single-line. A renderer's "what happens if `.name()` is empty?"
branch becomes unreachable.

The single `Result` return point in the library is `build()` — every
builder's `build()` returns `Result<T, Error>` because that's where
parsing happens. After `build()`, the type tells you everything is
valid.

## Why newtypes are `Copy`

`Selection` wraps `(index: usize, count: usize)`. `Cadence` wraps a
`u16`. `Tick` wraps a `u32`. All three derive `Copy`.

Per counterintuitive-patterns #7: **if every field is `Copy`, your
struct should be `Copy`**. `Selection::new(count)` instead of
`Selection::new(count).clone()` is the difference between a menu
crate and a menu trap. State machines pass these values around
frequently; making them `Copy` removes a category of borrow-checker
friction for callers.

## Pragmatic `const fn`

Lint rules from the project standard:

- `missing_const_for_fn` is **denied** (with allowed exceptions for
  functions whose const-elaboration is blocked by traits not yet
  const-stable in Rust 1.98 — `Index`, `Ord`, `PartialEq`, etc.).
- `expect(clippy::missing_const_for_fn, reason = "...")` is used where
  the constraint is real.

This means most getters in cornichon (`selection()`,
`theme()`, `cadence()`, `frame_count()`, etc.) are `const fn`. They
work in const contexts — including for compile-time tables or
generic composition.

A few specific cases that **couldn't** be const (`from_name`,
`frame_at`) are documented inline with the `rust-lang/rust#143874`
tracker issue link — they're sitting ducks for the day those traits
land as stable const.

## Default for builders

Each `XxxBuilder` derives [`Default`] where it can:

```rust
#[derive(Debug, Default, Clone)]
pub struct SlashMenuBuilder { … }
```

When the default matters semantically (`SpinnerBuilder.theme` should
be `Theme::default()` which is `CLAUDE`), the field type's own
`Default` impl provides it. This keeps builder construction cheap and
makes `.theme(theme)` the only theme-related setter needed.

## Testing

[`See ./testing.md`](./testing.md).
