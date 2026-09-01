# Introduction

## What is cornichon?

cornichon (named for the small pickled cucumber) is a Rust library of **composable
terminal-UI menu patterns** and an **exhaustive catalog of CLI spinner styles**,
visually themed after the menu UIs of Claude Code, OpenAI Codex, and Grok.

It packages two surfaces:

| Surface      | Widgets / Styles                                                                 |
| ------------ | -------------------------------------------------------------------------------- |
| **Menus**    | `SlashMenu`, `SelectMenu`, `PermissionPrompt`, `ProjectPicker`, `ShortcutsModal` |
| **Spinners** | 61 animation styles across dots, line, geometry, toggle, bounce, pulse, emoji, … |

Both surfaces integrate with [`ratatui`] via the standard `frame.render_widget(&menu_or_spinner, area)`
call. Consumers don't need to learn a new widget trait — cornichon implements
`Widget` for `&Menu` and `&Spinner` so the existing ratatui render loop works
unchanged.

## Design philosophy

> **Be a tiny, boring, well-typed piece of infrastructure that solves one thing well.**

Specifically:

1. **Pure-domain state machines, framework-agnostic rendering.** Every menu's
   `update()` returns a `MenuAction` enum. The state machine has zero
   knowledge of `ratatui`. Renderers (currently: `ratatui`) live in
   `adapters/` and are replaceable.

2. **Total `update()` semantics.** Every key event returns a `MenuAction`.
   Invalid events are `MenuAction::None`, not `Err`. Menus can never enter an
   "invalid" state — they always have a valid cursor, and they always have
   something to draw.

3. **Parse, don't validate.** Raw `&str` input is funneled through
   `TryFrom<(&str, &str)>` on [`MenuItem`](https://docs.rs/cornichon/latest/cornichon/struct.MenuItem.html)
   / [`AnnotatedItem`](https://docs.rs/cornichon/latest/cornichon/struct.AnnotatedItem.html)
   at construction time. After construction, those types are
   invariant-carrying — empty names, newlines-in-names, etc. are _unrepresentable_.

4. **Standards-compliant Rust.** No `.unwrap()` / `.expect()` / `panic!()` —
   even in tests. Newtypes wrapping `Copy` primitives are themselves `Copy`.
   `#[expect(...)]` over `#[allow(...)]`. `const fn` wherever feasible. The
   internal layout is the standard `src/{domain,ports,application,adapters}`.

5. **No runtime dependencies beyond ratatui + crossterm.** No async runtime,
   no `Instant::now()` inside the library, no background threads. The
   spinners are deterministic — drives happen in the caller's event loop.

## Inspiration

The visual designs are taken from the [`brainless`] shadcn/ui registry (the
Claude / Codex / Grok UIs). The widget architecture borrows from OpenTUI's
`SelectRenderable` (option list, configurable theme, key bindings, scroll
indicator, wrap-around, fast scroll, set-style-swap).

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
