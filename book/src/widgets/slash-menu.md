# SlashMenu

A **filterable `/command` palette** — the closest analogue of Claude Code's
slash-command UI in the brainless registry.

```text
> /cle   <-- active row, dimmed background
  /agents     Manage subagents for specialized tasks
  /clear      Clear conversation history and free up context  ← selected
  /compact    Summarize the conversation to save context
```

A `Query` line sits at the bottom of the menu; typing into it narrows the
visible items to those whose `name` (with any leading `/` stripped)
starts with the query, case-insensitively.

## Construction

```rust
use cornichon::prelude::*;

let mut menu = SlashMenu::builder()
    .items([
        MenuItem::try_from(("/agents", "Manage subagents"))?,
        MenuItem::try_from(("/clear", "Clear history"))?,
        MenuItem::try_from(("/compact", "Summarize"))?,
    ])
    .build()?;

// Or with the Claude default command list:
let menu = SlashMenu::with_defaults()?;
```

## API

### `SlashMenu::update(&mut self, event: KeyEvent) -> MenuAction`

The single entry point for advancing the state machine.

| Key                      | Effect                                                          |
| ------------------------ | --------------------------------------------------------------- |
| <kbd>↑</kbd>             | Move cursor up in the filtered view (wraps at top)              |
| <kbd>↓</kbd>             | Move cursor down in the filtered view (wraps at bottom)         |
| <kbd>Enter</kbd>         | Return `MenuAction::Selected(item)` for the highlighted command |
| <kbd>Esc</kbd>           | Return `MenuAction::Cancelled`                                  |
| <kbd>Backspace</kbd>     | Pop one character off the query; re-runs filter                 |
| Printable char (no Ctrl) | Append to the query; re-runs filter                             |
| Anything else            | `MenuAction::None` (no-op)                                      |

`update()` is **total** — it never panics, never returns `Err`. An invalid
event is just `MenuAction::None`.

### Filtering behavior

The filter is a **prefix match** on the item name, case-insensitive, with
any leading `/` stripped from both the query and the item names. An empty
query matches every item (preserves declaration order).

```rust
let mut menu = SlashMenu::with_defaults()?;
menu.update(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
menu.update(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
// `ag` matches `/agents` but not `/model` or `/review`.
assert_eq!(menu.filtered_indices(), &[0]); // index of `/agents` in defaults
```

### Accessors

```rust
menu.items();           // &[MenuItem]                — all items (unfiltered)
menu.query();           // &str                       — current filter query
menu.filtered_indices(); // &[usize]                  — original-item indices in display order
menu.selection();       // usize                      — cursor in the *filtered* view
menu.theme();            // Theme                      — active palette
```

## Customization

### Custom commands

```rust
let menu = SlashMenu::builder()
    .items([
        MenuItem::try_from(("/deploy", "Deploy to production"))?,
        MenuItem::try_from(("/rollback", "Revert the last deploy"))?,
    ])
    .theme(GROK)
    .build()?;
```

### Hotkey support

Pass a [`MenuItem`] with `.with_key(char)` to attach a hotkey:

```rust
let yes = MenuItem::try_from(("yes", "approve")).unwrap().with_key('y');
menu.update(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
// selection jumps to the item with key 'y'.
```

Hotkeys are case-sensitive single chars. Unmatched keys are silent no-ops
(no error beep).

### Custom theme

```rust
let menu = SlashMenu::builder()
    .theme(Theme {
        fg: Color::Cyan,
        fg_dim: Color::DarkGray,
        fg_muted: Color::Gray,
        accent: Color::Magenta,
        border: Color::Cyan,
        selected_bg: None,
        marker: "❯",
        marker_width: 1,
    })
    .build()?;
```

## When to use it

Use `SlashMenu` whenever you have a **fixed list of named actions** the
user invokes by typing. Slash commands are universally understood; the
filter pattern scales to hundreds of items without scrolling.

Don't use it for:

- **Hierarchical navigation** (use a tree-view widget, not in this crate yet).
- **Yes/no confirmations** (use [`PermissionPrompt`](./permission-prompt.md)).
- **Single-selection from a list of objects** (use [`SelectMenu`](./select-menu.md) — no text input).

[`MenuItem`]: https://docs.rs/cornichon/latest/cornichon/struct.MenuItem.html
