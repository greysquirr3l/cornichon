# ShortcutsModal

Grok CLI's **Keyboard Shortcuts modal** (<kbd>Ctrl+x</kbd>). A categorized
list — one row per category. Expanded categories show their
`action`/`keys` pairs on subsequent lines; collapsed categories collapse
to a single line.

```text
┌─ Keyboard Shortcuts ──────────────────[✗]┐
│ / to search                              │
├─────────────────────────────────────────┤
│ ◆ Essentials                             │ ← expanded: 8 rows follow
│     Send                       Enter    │
│     Focus scrollback           Tab      │
│     Cancel turn           Ctrl+c         │
│   > …                                   │ ← selected (collapsed others)
│   › Essentials (8)                         │ ← collapsed example
├─────────────────────────────────────────┤
│ ▷ Input (5)                               │
│ ▷ Session (3)                             │
└─────────────────────────────────────────┘
```

## Construction

The defaults match the brainless capture:

```rust
use cornichon::prelude::*;

let mut modal = ShortcutsModal::with_defaults()?;
```

Each default group is a `(&str /* id */, &[(&str /* action */, &str /* keys */)])`:

- `essentials`, `input`, `session` — populated with real data
- `nav`, `actions`, `panels`, `dashboard` — empty with a `count` hint

For your own groups:

```rust
use cornichon::application::shortcuts::{ShortcutsGroup, ShortcutsItem};

let modal = ShortcutsModal::builder()
    .groups([
        ShortcutsGroup::new(
            "edit",
            "Edit",
            vec![
                ShortcutsItem::new("Undo",       "Ctrl+z"),
                ShortcutsItem::new("Redo",       "Ctrl+Shift+z"),
                ShortcutsItem::new("Save",       "Cmd+s / Ctrl+s"),
            ],
        ),
        ShortcutsGroup::new(
            "nav",
            "Navigation",
            vec![],  // empty groups OK
        ).with_count(7),  // display " (7)" when collapsed
    ])
    .build()?;
```

## API

### `ShortcutsModal::update(&mut self, event: KeyEvent) -> MenuAction`

| Key                             | Effect                                                               |
| ------------------------------- | -------------------------------------------------------------------- |
| <kbd>↑</kbd>                    | Move group cursor up                                                 |
| <kbd>↓</kbd>                    | Move group cursor down                                               |
| <kbd>Enter</kbd> / <kbd>→</kbd> | **Expand** the current group                                         |
| <kbd>←</kbd>                    | **Collapse** the current group (no-op if already collapsed)          |
| <kbd>Esc</kbd>                  | Collapse first; if already collapsed, return `MenuAction::Cancelled` |
| Anything else                   | `MenuAction::None`                                                   |

The two-press Esc pattern is deliberate: it mirrors Claude Code's
two-stage close (collapse-on-one-Esc, leave-on-second-Esc) and gives the
user a chance to back out without losing their expansion state.

### Accessors

```rust
modal.groups();       // &[ShortcutsGroup]
modal.selection();    // usize              — index of the highlighted group
modal.expanded();     // Option<usize>      — Some(idx) if a group is expanded, None otherwise
modal.theme();         // Theme
```

## Empty groups are permitted

Unlike the other builders, [`ShortcutsModal::builder::build()`] does **not**
require any groups. An empty modal renders nothing and every event is
`MenuAction::None` — useful for "fetch the help content first, then show
the modal once it's loaded" flows.

## When to use it

Use `ShortcutsModal` for **discoverability surfaces** — the screen that
appears when the user presses <kbd>?</kbd> or <kbd>F1</kbd>. It pairs
naturally with an
[`Input`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Input.html)
filter: as the user types into the filter, hide groups whose labels
don't match the prefix.

Don't use it for:

- **Help content longer than a couple of screens** (write proper docs
  elsewhere; the modal is for key bindings, not prose).
- **Lists that always need to be expanded** (use [`SelectMenu`](./select-menu.md)
  directly — modal nesting adds nothing).
