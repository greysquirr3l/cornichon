# Why cornichon doesn't implement `StatefulWidget`

A short note that's saving us from a design mistake.

[ratatui's `StatefulWidget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html)
exists for widgets whose **state is separate from the widget value**
— `ListState`, `ScrollbarState`, `TableState`. The trait signature is:

```rust,ignore
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

`ratatui::widgets::List::render` takes a `&mut ListState` because the
list itself doesn't own the cursor — the _caller_ owns it. That makes
sense when you have multiple widgets sharing a single source-of-truth
state (e.g., one `ListState` driving three mirrored `List` widgets in
different regions of the screen).

Cornichon's menus **own** their state by design. `SelectMenu` carries
its selection index inside the struct; the widget is stateful on its
own and does not need a separate `State` token. The trade-offs are:

| Owned state (cornichon)                               | Separate state (`StatefulWidget`) |
| ----------------------------------------------------- | --------------------------------- |
| One value per menu on screen                          | One state, many widgets share it  |
| Cursor resets when the menu drops                     | Caller controls lifetime          |
| Zero-cost construction (`MenuBuilder::new().build()`) | Cost: build a `State` first       |
| Mutates via `&mut self` on `update`                   | State mutates via the render loop |
| Builder pattern (`new -> with_label -> with_style`)   | State built externally            |

For a selection-menu UI (the entire cornichon surface area today),
owned state is simpler and more idiomatic. Adding `StatefulWidget`
impls would require splitting the selection out of each `Menu` type
into a parallel `MenuState` struct — invasive, loses the builder
pattern, and gains nothing for our use cases.

If you genuinely need shared state (e.g., one cursor driving two
menus), the right answer is a higher-level construct built **on top
of** cornichon, not a redesign inside it.

## What we _do_ provide

Every menu implements `Widget for &Menu`, the standard ratatui trait:

```rust,ignore
fn render(self, area: Rect, buf: &mut Buffer);
```

That plays well with any `Frame`, including `Frame::render_widget`,
`Paragraph`, or composed layouts. The state lives inside the menu;
to share it, wrap the menu in `Rc<RefCell<…>>` or `Arc<Mutex<…>>` at
the call site.
