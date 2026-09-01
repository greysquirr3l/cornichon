# Spinners · Style catalog

61 distinct animations, organized into 9 families. Every variant is
identified by a stable snake-case name (matching
[cli-spinners](https://github.com/sindresorhus/cli-spinners) where
applicable).

```rust
use cornichon::prelude::*;

let style = SpinnerStyle::try_from("bouncingBar")?;
let style = SpinnerStyle::from_name("bouncingBar").unwrap();
let name = SpinnerStyle::Dots.name(); // "dots"
```

Frames live in `&'static [&'static str]` tables — no allocation, no
clones, just `O(1)` pointer math.

---

## Dots (braille)

The most popular spinner family in modern CLIs. Each frame is 1-3
braille characters that visually "swirl" through the dot matrix.

| Variant  | Frames | Cadence | Default? | Notes                     |
| -------- | ------ | ------- | -------- | ------------------------- |
| `Dots`   | 10     | 80 ms   | ✅       | The classic `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`  |
| `Dots2`  | 11     | 80 ms   |          | Clockwise variant         |
| `Dots3`  | 11     | 80 ms   |          | Counter-clockwise         |
| `Dots4`  | 7      | 80 ms   |          | Grow-once (`⠁⠃⠇⠧⠷⠿⣿`)     |
| `Dots5`  | 7      | 80 ms   |          | Grow-then-stay            |
| `Dots6`  | 10     | 80 ms   |          | Top-down sweep            |
| `Dots7`  | 6      | 80 ms   |          | Middle-out                |
| `Dots8`  | 6      | 80 ms   |          | Pulse-out                 |
| `Dots9`  | 8      | 80 ms   |          | Staircase (`⠁⠃⠄⠂`)        |
| `Dots10` | 8      | 80 ms   |          | Top-down triangle braille |
| `Dots11` | 8      | 80 ms   |          | Staircase reverse         |
| `Dots12` | 8      | 80 ms   |          | Staircase grow            |

## Line

ASCII-only — works in any terminal, no Unicode support required.

| Variant | Frames | Cadence | Frames       |
| ------- | ------ | ------- | ------------ |
| `Line`  | 4      | 130 ms  | `\| / - \`   |
| `Line2` | 5      | 100 ms  | `- \ \| / -` |

## Geometry

Single-character geometric shapes that rotate.

| Variant         | Frames | Glyphs                        |
| --------------- | ------ | ----------------------------- |
| `Arc`           | 6      | `◜◠◝◞◡◟`                      |
| `Arrow`         | 8      | `←↖↑↗→↘↓↙` (all 8 directions) |
| `Arrow2`        | 4      | `←↑→↓` (4 directions)         |
| `Arrow3`        | 2      | `▹▸`                          |
| `Quadrants`     | 4      | `▖▘▝▗`                        |
| `Triangle`      | 4      | `◢◣◤◥`                        |
| `SquareCorners` | 4      | `◰◳◲◱`                        |
| `CircleHalves`  | 4      | `◐◑◒◓`                        |

## Toggle / Block

Two-frame morphs between a "filled" and "empty" variant of the same shape.

| Variant   | Frames | Glyphs |
| --------- | ------ | ------ |
| `Toggle`  | 2      | `▮▯`   |
| `Toggle2` | 2      | `▫▪`   |
| `Toggle3` | 2      | `□■`   |
| `Toggle4` | 2      | `◌◍`   |
| `Toggle5` | 2      | `▰▱`   |
| `Toggle6` | 2      | `◽◾` |

## Bouncing / Bars

Indeterminate-progress animations — bars that fill, balls that bounce.

| Variant          | Frames | Notes                                      |
| ---------------- | ------ | ------------------------------------------ |
| `BouncingBar`    | 14     | `[    ]` → `[=   ]` → `[==  ]` → … (wraps) |
| `BouncingBall`   | 10     | `( ●    )` → `(  ●   )` → `(   ●  )` → …   |
| `Hamburger`      | 3      | `☱☲☴`                                   |
| `GrowVertical`   | 8      | `▁▂▃▄▅▆▇█` (stair)                         |
| `GrowHorizontal` | 8      | `▏▎▍▌▋▊▉█`                                 |

## Pulse / Progress

| Variant  | Frames | Notes                            |
| -------- | ------ | -------------------------------- |
| `Pulse`  | 5      | `█▒` → `▒▒` → `▒░` → `░░` → `░█` |
| `Points` | 3      | `∙•●`                            |
| `Binary` | 8      | `0 1 0 1 _ 1 0 1`                |
| `Pipe`   | 5      | `┤┘┐└┌`                          |
| `Layer`  | 3      | `- = ≡`                          |

## Noise / Decoration

| Variant     | Frames | Glyphs                         |
| ----------- | ------ | ------------------------------ |
| `Aesthetic` | 3      | `▓▒░`                          |
| `Material`  | 9      | `⣾⣽⣻⢿⡿⣟⣯⣷` (10-frame rotation) |
| `Material2` | 6      | `⣷⣯⣟⡿⢿⣻⣽⣾` (reverse)           |
| `Noise`     | 4      | `⠁⠂⠄⠂`                         |
| `Squish`    | 2      | `⊂⊃`                           |
| `Star`      | 5      | `✶✸✺✹✷`                        |

## Emoji

Variants that require Unicode emoji support. Some shells / fonts
(`alacritty`, `kitty`) render these beautifully; others fall back to
boxes.

| Variant     | Frames | Cadence | Glyphs                     |
| ----------- | ------ | ------- | -------------------------- |
| `Clock`     | 12     | 130 ms  | `🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚🕛` |
| `Earth`     | 3      | 80 ms   | `🌍🌎🌏`                   |
| `Moon`      | 8      | 80 ms   | `🌑🌒🌓🌔🌕🌖🌗🌘`         |
| `Weather`   | 4      | 80 ms   | `☀️⛅️☁️🌧️`                 |
| `Christmas` | 3      | 80 ms   | `🎄🎁🎅`                   |
| `Monkey`    | 3      | 80 ms   | `🙈🙉🙊`                   |
| `Runner`    | 4      | 80 ms   | `🚶🏃💃🕺`                 |
| `Pong`      | 2      | 80 ms   | `▐▌`                       |
| `Shark`     | 3      | 80 ms   | `▐▌▐▌▐▌`                   |
| `Dqpb`      | 4      | 80 ms   | `- x + x`                  |
| `MindBlown` | 1      | 80 ms   | `🤯` (single-frame "loop") |
| `Speaker`   | 4      | 80 ms   | `🔊🔉🔈🔇`                 |
| `Heart`     | 5      | 80 ms   | `❤💗💖💝💘`                |

## Special

| Variant               | Frames | Notes                                                                                  |
| --------------------- | ------ | -------------------------------------------------------------------------------------- |
| `Bounce`              | 4      | `⠁⠂⠄⠂` (single-character bounce)                                                       |
| `SimpleDots`          | 4      | `.` `..` `...` `   `                                                                   |
| `SimpleDotsScrolling` | 6      | `.` `..` `...` ` ..` `  .` `   `                                                       |
| `SimpleDotsTyping`    | 6      | Same frames as `SimpleDotsScrolling`, but psychophysics says it feels more like typing |

## Picking a style

A few rules of thumb:

1. **Default to braille dots.** A terminal user sees them every day in
   Claude Code / Codex / Grok / cargo / rustc. They render correctly
   on every modern terminal.

2. **Use ASCII (`Line`) only when you can't trust the terminal.** CI logs,
   tmux with weird fonts, `script -q` capture. Otherwise dot bars look
   better.

3. **Reserve emoji variants for splash screens and progress overlays**, not
   menu spinners — emoji take 2 cells, which makes alignment fragile if
   the spinner is in a fixed-width column next to text.

4. **Don't ship `MindBlown` for actual use.** It's there because
   cli-spinners has it, and removing a style is a breaking change.

## `all_names()`

Useful for "pick random" logic or for populating a settings UI:

```rust
let count = SpinnerStyle::all_names().len(); // 61
let random = SpinnerStyle::from_name(
    SpinnerStyle::all_names()[fastrand::usize(..count)],
)
.unwrap();
```

[`SpinnerStyle::all_names`]: https://docs.rs/cornichon/latest/cornichon/enum.SpinnerStyle.html#method.all_names
