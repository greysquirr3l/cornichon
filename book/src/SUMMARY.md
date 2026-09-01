# Summary

[Introduction](./introduction.md)

- [What is cornichon?](./introduction.md#what-is-cornichon)
- [Design philosophy](./introduction.md#design-philosophy)
- [Inspiration](./introduction.md#inspiration)

---

# Getting Started

- [Install](./install.md)
- [Quickstart](./quickstart.md)

---

# Widgets

- [SlashMenu](./widgets/slash-menu.md)
- [SelectMenu](./widgets/select-menu.md)
- [PermissionPrompt](./widgets/permission-prompt.md)
- [ProjectPicker](./widgets/project-picker.md)
- [ShortcutsModal](./widgets/shortcuts-modal.md)

---

# Spinners

- [Overview](./spinners/overview.md)
  - [Cadence](./spinners/overview.md#cadence)
  - [Tick](./spinners/overview.md#tick)
  - [Total animation semantics](./spinners/overview.md#total-animation-semantics)
- [Style catalog](./spinners/styles.md)
  - [Dots (braille)](./spinners/styles.md#dots-braille)
  - [Line](./spinners/styles.md#line)
  - [Geometry](./spinners/styles.md#geometry)
  - [Toggle / Block](./spinners/styles.md#toggle--block)
  - [Bouncing / Bars](./spinners/styles.md#bouncing--bars)
  - [Pulse / Progress](./spinners/styles.md#pulse--progress)
  - [Noise / Decoration](./spinners/styles.md#noise--decoration)
  - [Emoji](./spinners/styles.md#emoji)
  - [Special](./spinners/styles.md#special)
- [Animation timing](./spinners/animation.md)
  - [`advance()` vs `tick_at()`](./spinners/animation.md#advance-vs-tick_at)
  - [Driving from `crossterm`](./spinners/animation.md#driving-from-crossterm)
  - [Driving from a global clock](./spinners/animation.md#driving-from-a-global-clock)

---

# Themes

- [Built-in themes](./themes.md)
  - [Claude (`CLAUDE`)](./themes.md#claude-claude)
  - [Codex (`CODEX`)](./themes.md#codex-codex)
  - [Grok (`GROK`)](./themes.md#grok-grok)
- [Custom themes](./themes.md#custom-themes)

---

# Project

- [Architecture](./architecture.md)
  - [Layered layout](./architecture.md#layered-layout)
  - [Pragmatic on ratatui](./architecture.md#pragmatic-on-ratatui)
  - [Total `update` semantics](./architecture.md#total-update-semantics)
  - [Parse, don't validate](./architecture.md#parse-dont-validate)
- [Testing](./testing.md)
  - [State-machine unit tests](./testing.md#state-machine-unit-tests)
  - [Buffer render tests](./testing.md#buffer-render-tests)

---

# Contributing

- [Widget cookbook](./contributing/cookbook.md)
- [Why no `StatefulWidget?`](./contributing/stateful.md)

---

# Appendices

- [Roadmap](./appendices/roadmap.md)
- [Changelog](./appendices/changelog.md)
