# ProjectPicker

The **"Run in a project directory?"** chooser from Grok CLI. Like a
[`SelectMenu`] but with a trailing **"custom" option** (default hotkey:
<kbd>z</kbd>) that returns [`MenuAction::Activated`] instead of
[`MenuAction::Selected`]. The caller is expected to switch into a
free-text input mode on activation.

```text
┃ Run in a project directory?
┃ This gives full context of your codebase for better results.
┃
┃   1 brainless    ~/dev/brainless               (current)
┃   2 freestyle-cloudstate  ~/Documents/...    (5h ago)
┃   3 Conduit      ~/Documents/Conduit           (5h ago)
┃ > z Type your answer here                     ← selected
```

The left-border card style comes from the Grok theme.

## Construction

```rust
use cornichon::prelude::*;

let mut picker = ProjectPicker::with_defaults()?;
```

Default projects are the three captured in the brainless registry:

| Index | `key`       | `name`                               | `path`                   | annotation       |
| ----- | ----------- | ------------------------------------ | ------------------------ | ---------------- |
| 0     | `Some('1')` | `brainless`                          | `~/dev/brainless`        | —                |
| 1     | `Some('2')` | `freestyle-cloudstate`               | `~/Documents/GitHub/...` | `Some("5h ago")` |
| 2     | `Some('3')` | `Conduit`                            | `~/Documents/Conduit`    | `Some("5h ago")` |
| —     | `Some('z')` | _(trailing "Type your answer here")_ | —                        | —                |

For your own list:

```rust
let picker = ProjectPicker::builder()
    .projects([
        AnnotatedItem::try_from(("brainless",   "~/dev/brainless",   Some("current")))?,
        AnnotatedItem::try_from(("fooproject",  "~/dev/fooproject",  Some("3h ago")))?,
    ])
    .custom_label("Enter a path manually")  // override the trailing option's text
    .build()?;
```

## API

### `ProjectPicker::update(&mut self, event: KeyEvent) -> MenuAction`

| Key                       | Effect                                                                              |
| ------------------------- | ----------------------------------------------------------------------------------- |
| <kbd>↑</kbd>              | Move cursor up (wraps)                                                              |
| <kbd>↓</kbd>              | Move cursor down (wraps)                                                            |
| <kbd>1</kbd>…<kbd>9</kbd> | Jump to that project                                                                |
| <kbd>z</kbd>              | Jump to the "custom" option                                                         |
| <kbd>Enter</kbd>          | `MenuAction::Selected(item)` for projects, **`MenuAction::Activated`** for "custom" |
| <kbd>Esc</kbd>            | `MenuAction::Cancelled`                                                             |

The selection index covers **both** projects and the trailing custom option,
so `picker.selection()` ranges over `0..=projects.len()`. Use
[`is_custom_selected`](https://docs.rs/cornichon/latest/cornichon/struct.ProjectPicker.html#method.is_custom_selected)
to disambiguate.

### Dispatch on action

```rust
match picker.update(key) {
    MenuAction::Selected(item) => {
        run_with_project(&item.name, &item.description);
    }
    MenuAction::Activated => {
        // Switch to a free-text input mode (e.g., another textbox
        // widget or a ratatui `Input` from `ratatui::widgets::InputState`).
        enter_free_text_mode();
    }
    MenuAction::Cancelled => {
        // User backed out without picking.
    }
    MenuAction::None => {}
}
```

## Why it's not `SelectMenu` with an extra entry

The `Activated` action variant exists for exactly this case — letting
the caller distinguish "user picked one of these" from "user wants to
enter something not on the list". Folding that into `MenuAction::Selected`
would lose information; folding into `MenuAction::Cancelled` would be
even worse (the user _did_ pick something).

`ProjectPicker` is the only menu type that returns `Activated` today.
If a future widget needs the same dispatch (a filterable picker with a
"create new" entry, say), it'll get its own type with its own
documentation — but the `Activated` action variant is the convention.

## When to use it

Use `ProjectPicker` for any **"pick from a short list, or specify your
own"** flow: project directories, themes, languages, file paths, agent
names, etc.

Don't use it for:

- **Pure pick-from-list without a custom fallback** (use [`SelectMenu`]).
- **Anywhere the "custom" path is the primary use case** (just use a
  text input directly — no menu needed).

[`SelectMenu`]: ./select-menu.md
[`MenuAction::Activated`]: https://docs.rs/cornichon/latest/cornichon/enum.MenuAction.html#variant.Activated
[`MenuAction::Selected`]: https://docs.rs/cornichon/latest/cornichon/enum.MenuAction.html#variant.Selected
