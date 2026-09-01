# SelectMenu

A **basic single-select list** — the simplest menu in the crate. No
filtering, no metadata column, no header chrome. Just an item list, a
cursor, and arrow / Enter / Esc.

```text
Apple
Banana            ← selected
Cherry
Date
```

## Construction

```rust
use cornichon::prelude::*;

let mut menu = SelectMenu::builder()
    .title("Pick a fruit")
    .items([
        MenuItem::try_from(("Apple",  "A red fruit"))?,
        MenuItem::try_from(("Banana", "A yellow fruit"))?,
        MenuItem::try_from(("Cherry", "A red stone fruit"))?,
    ])
    .build()?;
```

A `title` is optional. The builder rejects an empty list:

```rust,ignore
SelectMenu::builder().build()  // returns Err(Error::EmptyMenu)
```

## API

### `SelectMenu::update(&mut self, event: KeyEvent) -> MenuAction`

| Key                       | Effect                                 |
| ------------------------- | -------------------------------------- |
| <kbd>↑</kbd>              | Move cursor up (wraps)                 |
| <kbd>↓</kbd>              | Move cursor down (wraps)               |
| <kbd>Enter</kbd>          | `MenuAction::Selected(item)`           |
| <kbd>Esc</kbd>            | `MenuAction::Cancelled`                |
| Char (matches `item.key`) | Jump to that item (silent if no match) |

### Accessors

```rust
menu.items();      // &[MenuItem]
menu.selection();  // usize       — cursor index (always 0 if list is empty)
menu.title();       // Option<&str>
menu.theme();       // Theme
```

## Customization

`SelectMenu` is intentionally minimal. To customize appearance or add
features, swap to a richer menu type (`SlashMenu` for filtering,
`ProjectPicker` for an annotation column, etc.). If you need
`SelectMenu`-like behavior with a label, the `AnnotatedItem` value-object
plus the `selection`-by-key hotkeys give you the same APIs.

## When to use it

Use `SelectMenu` for **small, fixed lists** (≤ ~20 items) where the user
won't bother to type a filter. The benefit over a simple `<list>` UI is
the wrapped arrow-key navigation + the keyed selection that drops
straight into a `MenuAction` enum.

Don't use it for:

- **Anything filterable** (use [`SlashMenu`](./slash-menu.md)).
- **Approval flows** (use [`PermissionPrompt`](./permission-prompt.md)).
- **Lists > 50 items** — there's no scrollbar; build a virtualized list
  or split into a tabbed view.
