# PermissionPrompt

Claude Code's **"Do you want to proceed?"** chooser. A fixed header
(`title`, `command`, `question`), a numbered option list with
pre-attached hotkeys `1`/`2`/`3`, and a focus ring on the highlighted
option.

```text
┌─ Bash command ─────────────────────────────────────────┐
│ rm -rf node_modules                                    │
│                                                       │
│ Do you want to proceed?                               │
│                                                       │
│   1. Yes                                              │
│ > 2. Yes, and don't ask again this session ← selected  │
│   3. No, and tell Claude what to do (esc)            │
└───────────────────────────────────────────────────────┘
```

The rose-colored border is part of the Claude theme; swap themes (e.g.,
to `CODEX`) and you'll get a near-white border with a cream accent.

## Construction

The defaults match the brainless capture exactly:

```rust
use cornichon::prelude::*;

let mut prompt = PermissionPrompt::with_defaults()?;
// or, equivalently:
let mut prompt = PermissionPrompt::builder().with_defaults()?.build()?;
```

For custom flows:

```rust
let prompt = PermissionPrompt::builder()
    .title("Tool call: Bash")
    .command("git push --force-with-lease")
    .question("Allow this push?")
    .items([
        MenuItem::try_from(("Allow once",  "1"))?,
        MenuItem::try_from(("Allow for this session", "2"))?,
        MenuItem::try_from(("Reject",      "3"))?,
    ])
    .theme(CODEX)
    .build()?;
```

## API

### `PermissionPrompt::update(&mut self, event: KeyEvent) -> MenuAction`

| Key                                    | Effect                                            |
| -------------------------------------- | ------------------------------------------------- |
| <kbd>↑</kbd>                           | Move cursor up                                    |
| <kbd>↓</kbd>                           | Move cursor down                                  |
| <kbd>1</kbd>/<kbd>2</kbd>/<kbd>3</kbd> | Jump to that option (via the pre-attached hotkey) |
| <kbd>Enter</kbd>                       | `MenuAction::Selected(item)`                      |
| <kbd>Esc</kbd>                         | `MenuAction::Cancelled`                           |

The hotkeys default to `1`/`2`/`3` (matching Claude Code's numbering).
Override by passing `MenuItem::try_from((label, hotkey_string))` with
your own digit, or drop the second tuple field for an unkeyed option.

### Accessors

```rust
prompt.title();     // &str
prompt.command();   // &str
prompt.question();  // &str
prompt.items();     // &[MenuItem]
prompt.selection(); // usize
prompt.theme();     // Theme
```

## Why it's separate from `SelectMenu`

`PermissionPrompt` is structurally a [`SelectMenu`] with three extra
header lines and pre-attached numeric hotkeys. It exists as its own
type because:

1. **Discoverability** — `PermissionPrompt` literally says what it is.
   A type-driven grep of the codebase (`grep -r PermissionPrompt`)
   finds every approval flow instantly.
2. **Header layout** — the build-up of `title` → `command` → `question`
   is opinionated and shouldn't be the default for every list.
3. **Hotkey convention** — Claude Code numbers its approval options. Hard-coding
   `1`/`2`/`3` lets us lint that convention without making every caller
   pass it manually.
4. **Renderer hooks** — a real shell might want a different background
   color or border style for permission prompts (red-bordered, the way
   Claude does). A dedicated type makes that a single match arm in the
   renderer.

A future `YesNo` or `Confirm` widget for boolean prompts would be a
subclass of the same pattern (a 2-option `PermissionPrompt` with a
default of "Yes, and don't ask again" and a footer hint).

## When to use it

Use `PermissionPrompt` for **any user-confirmation dialog** where the
cost of running the action is non-trivial (shell command, file write,
network request, payment, etc.). The fixed 1/2/3 numbering makes the
question scannable and the answer cheap.

Don't use it for:

- **Two-option binary choices** where you don't want a numbered menu
  (consider a tiny wrapper that builds a `PermissionPrompt` with `["Yes", "No"]`).
- **Long lists of confirmable options** (use [`SelectMenu`] with a
  styled title instead).

[`SelectMenu`]: ./select-menu.md
