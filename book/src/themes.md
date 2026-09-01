# Themes

A cornichon **theme** is the four (or five) colors plus an active-row
marker glyph that menus and spinners use to render their rows.

```rust
pub struct Theme {
    pub fg: Color,          // Primary text (selected row label, headers)
    pub fg_dim: Color,      // Secondary text (descriptions, hints)
    pub fg_muted: Color,    // Tertiary text (unselected rows, footer)
    pub accent: Color,      // Focus rings, active borders
    pub border: Color,      // Outer frame color
    pub selected_bg: Option<Color>,  // Optional tint behind the selected row
    pub marker: &'static str,        // Active-row marker glyph (e.g. "❯")
    pub marker_width: u16,            // Width of the marker column in cells
}
```

`Theme` is `Copy` (every field is `Copy`), so passing it around is free.

## Built-in themes

Three themes ship as `const`s, named after the CLI they were captured from.

### Claude (`CLAUDE`)

```rust
pub const CLAUDE: Theme = Theme {
    fg:      Color::Rgb(0xaf, 0xd7, 0xff), // #afd7ff (38;5;153)
    fg_dim:  Color::Rgb(0xc0, 0xca, 0xf5), // #c0caf5 (Tokyo Night "foreground")
    fg_muted: Color::Rgb(0x94, 0x94, 0x94), // #949494 (38;5;246)
    accent:  Color::Rgb(0xcd, 0x69, 0x4a), // #cd694a (38;5;131) — rose
    border:  Color::Rgb(0xcd, 0x69, 0x4a), // rose border
    selected_bg: None,                     // selection by foreground + marker only
    marker: "❯",                          // U+276F (heavy right-pointing)
    marker_width: 1,
};
```

Claude Code's tints: pale-cyan active, dimmed-gray inactive, rose
border around permission prompts.

### Codex (`CODEX`)

```rust
pub const CODEX: Theme = Theme {
    fg:      Color::Rgb(0xed, 0xed, 0xed), // #ededed — near-white
    fg_dim:  Color::Rgb(0xed, 0xed, 0xed), // (fg = fg_dim; descriptions inherit)
    fg_muted: Color::Rgb(0x7a, 0x7a, 0x7a), // #7a7a7a — medium gray
    accent:  Color::Rgb(0xf6, 0xe2, 0xb7), // #f6e2b7 — cream
    border:  Color::Rgb(0x7a, 0x7a, 0x7a), // gray border (matches dim text)
    selected_bg: None,
    marker: "›",                          // U+203A (single right-pointing angle quote)
    marker_width: 1,
};
```

OpenAI Codex's high-contrast palette: bright on dark, cream accent.

### Grok (`GROK`)

```rust
pub const GROK: Theme = Theme {
    fg:      Color::Rgb(0xe1, 0xe1, 0xe1), // #e1e1e1 — off-white
    fg_dim:  Color::Rgb(0x8b, 0x8b, 0x90), // #8b8b90 — gray-purple
    fg_muted: Color::Rgb(0x6c, 0x6c, 0x6c), // #6c6c6c — darker gray
    accent:  Color::Rgb(0xe1, 0xe1, 0xe1), // (matches fg for clean look)
    border:  Color::Rgb(0x80, 0x80, 0x80), // #808080 (38;5;8) — silver
    selected_bg: None,
    marker: "❯",                          // U+276F (heavy right-pointing)
    marker_width: 2,                       // wider gutter than Claude
};
```

Grok's left-border card style is implemented by leaving `selected_bg =
None` and relying on a left `│` rule in the row gutter when needed.

### Default

`Theme::default()` returns `CLAUDE` — the most-tested palette.

## Custom themes

A theme is a struct literal — no runtime builder. Themes are typically
declared as `const`s at the application level:

```rust
use cornichon::domain::Theme;
use ratatui::style::Color;

pub const SOLARIZED_LIGHT: Theme = Theme {
    fg:      Color::Rgb(0x00, 0x2b, 0x36), // base03
    fg_dim:  Color::Rgb(0x58, 0x6e, 0x75), // base00
    fg_muted: Color::Rgb(0x93, 0xa1, 0xa1), // base1
    accent:  Color::Rgb(0xb5, 0x89, 0x00), // yellow
    border:  Color::Rgb(0xee, 0xe8, 0xd5), // base2
    selected_bg: Some(Color::Rgb(0xfd, 0xf6, 0xe3)), // base3 — subtle selection tint
    marker:  "›",
    marker_width: 1,
};
```

Then thread it into a menu:

```rust
let mut menu = SlashMenu::builder()
    .theme(SOLARIZED_LIGHT)
    .items(my_commands)
    .build()?;
```

### Tweaking existing themes

For one-off tweaks, copy the const and edit:

```rust
const MY_CLAUDE: Theme = Theme {
    border: Color::Magenta,    // override border color
    ..CLAUDE                   // fill in the rest
};
```

The `..` syntax lets you change only what differs, with the compiler
checking field presence for you.

### `selected_bg`

The optional `selected_bg` field tints the **full row** behind the
selected item. All three bundled themes leave this `None` (selection by
foreground + marker only) — modern TUIs prefer this look because tinted
backgrounds can fight with terminal colorscheme interpretation. Use
`Some(Color::…)` if you want the OpenAI-Codex-style full-row tint.

## Why `marker` is `&'static str`

The marker glyph is part of the theme because the **active-row glyph is
visual identity**. Claude Code uses `❯` (heavy right-pointing), Codex
uses `›` (single right-angle quote), Grok uses `❯` but with a 2-cell
gutter. Keeping them as `&'static str` in the theme struct means:

- No allocation when building a menu (themes are `Copy`).
- No formatting cost on every render frame.
- Spinner rows can render the marker glyph at the same column as menu
  rows, so a `ProjectPicker` next to a `Spinner` lines up.

`marker_width` is `u16` rather than `u8` because ratatui's `Rect`
widths are `u16`.
