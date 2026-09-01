# Quickstart

A working `SlashMenu` in ~50 lines of ratatui wiring, end-to-end.

```rust,no_run
use cornichon::prelude::*;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::execute;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> io::Result<()> {
    // 1. Set up the terminal.
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // 2. Build the menu.
    let mut menu = SlashMenu::with_defaults()?;

    // 3. Drive it.
    let chosen = loop {
        terminal.draw(|f| {
            // Layout the menu at the top of the screen, leaving room for
            // any follow-up prompts below.
            let area = f.area();
            f.render_widget(&menu, Rect::new(0, 0, area.width, 10));
        })?;

        let Event::Key(key) = event::read()? else { continue };

        match menu.update(key) {
            MenuAction::Selected(item) => break Some(item.name),
            MenuAction::Cancelled => break None,
            // No-op events (typing into the filter, arrow keys, etc.)
            // land here — just redraw on the next loop iteration.
            MenuAction::None | MenuAction::Activated => {}
        }
    };

    // 4. Tear down.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("\nselected: {:?}", chosen);
    Ok(())
}
```

That `SlashMenu::with_defaults()?` constructs the Claude-style default command
list (`/agents`, `/clear`, `/compact`, `/init`, `/model`, `/review`). Type
after `/` to filter; arrow keys to navigate; Enter to confirm; Esc to cancel.

The only "cornichon" lines in the example above are `with_defaults()` and
`menu.update(key)`. Everything else is stock ratatui + crossterm. That's
deliberate — cornichon is a widget library, not a framework.

## What's happening?

1. **`SlashMenu::with_defaults()`** — builder shortcut equivalent to
   `SlashMenu::builder().with_defaults()?.build()?`. The `?` propagates any
   failure (currently only `EmptyMenu`, which can't trigger from the
   defaults).

2. **`menu.update(key)`** — the **only** API surface for advancing the
   menu's state machine. Returns a [`MenuAction`] — `None` for no-op
   events (typing into the filter), `Selected(item)` on Enter, `Cancelled`
   on Esc, `Activated` when the caller should swap input modes (e.g.,
   picking the "custom" option in `ProjectPicker`).

3. **`f.render_widget(&menu, area)`** — the standard ratatui render call.
   cornichon implements `Widget for &Menu`, so no special render method
   is needed.

4. **The whole loop is ~10 lines of glue**, all stock ratatui. cornichon
   gets out of your way once the menu's built.

## Try one of the examples

Each widget has a runnable demo. From the repo root:

```bash
cargo run -p cornichon-slash      # SlashMenu
cargo run -p cornichon-choice     # PermissionPrompt
cargo run -p cornichon-picker     # ProjectPicker with the trailing "custom"
cargo run -p cornichon-shortcuts # ShortcutsModal
cargo run -p cornichon-spinner   # 16 spinner styles animating at once
```

Press <kbd>q</kbd> or <kbd>Esc</kbd> in any of them to quit.

[`MenuAction`]: https://docs.rs/cornichon/latest/cornichon/enum.MenuAction.html
