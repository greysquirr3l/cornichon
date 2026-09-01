# Spinners · Overview

cornichon ships an **exhaustive catalog of CLI spinner styles** as a
single Rust enum — 61 variants covering the classic ASCII line,
braille `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`-style dots, geometry, blocks, bouncing bars,
pulse / progress, emoji, and a few "special" single-purpose frames.

Every spinner is **pure data + a const lookup** — there's no
`Instant::now()` inside the library, no background threads, no async. You
drive the animation; cornichon draws the current frame.

## Two pieces, one widget

The spinner crate is split into three pieces and one widget:

| Type                                                                                                                                               | Layer            | Role                                             |
| -------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------------------------------------------ |
| [`SpinnerStyle`](https://docs.rs/cornichon/latest/cornichon/enum.SpinnerStyle.html)                                                                | domain           | **What** animation — variant enum + frame table  |
| [`Cadence`](https://docs.rs/cornichon/latest/cornichon/struct.Cadence.html), [`Tick`](https://docs.rs/cornichon/latest/cornichon/struct.Tick.html) | domain           | **How fast** + **which frame**                   |
| [`Spinner`](https://docs.rs/cornichon/latest/cornichon/struct.Spinner.html)                                                                        | application      | State machine tying them together                |
| `Widget for &Spinner`                                                                                                                              | adapters/ratatui | Renders the current frame glyph + optional label |

Concretely:

```rust
use cornichon::prelude::*;

let spinner = Spinner::builder()
    .style(SpinnerStyle::Dots)
    .cadence(Cadence::NORMAL)  // 80 ms per frame
    .label("Loading")
    .theme(CLAUDE)
    .build();
```

### `Cadence`

```rust
pub struct Cadence(pub u16);  // milliseconds per frame

impl Cadence {
    pub const FAST:   Self = Self(50);
    pub const NORMAL: Self = Self(80);
    pub const SLOW:   Self = Self(130);
}
```

`Cadence` is the frame interval in milliseconds. Three named presets
cover ~95% of cases; build a custom cadence for everything else:

```rust
let cadence = Cadence(40);  // 25 fps — fast for active progress
let slow = Cadence::from_duration(Duration::from_millis(200));
```

### `Tick`

```rust
pub struct Tick(pub u32);
```

A monotonic frame counter. `Tick(0)` is the initial frame; the counter
wraps `u32` (effectively never — at 80ms cadence that's ~10 years of
uptime). The frame to draw at any moment is `frames[Tick.0 % frames.len()]`.

`Tick` is `Copy + Hash + Default`, per the newtypes-are-Copy rule.

### Total animation semantics

The core insight: **animation speed is independent of renderer frame
rate.** cornichon's spinner takes _total elapsed uptime_ (via
[`tick_at`](https://docs.rs/cornichon/latest/cornichon/struct.Spinner.html#method.tick_at))
and computes the frame index as `total_elapsed / cadence.millis()`. This
gives:

- **Deterministic output** for tests (no `Instant::now` inside the lib to flake).
- **Stable pacing** across host machines (same spin → same frame at
  same wall-clock second).
- **Decoupled render frequency** — re-draw the screen at 60fps or 5fps,
  the spinner shows the right glyph either way.

See [`./animation.md`](./animation.md) for the difference between
`tick_at()` (clock-driven) and `advance()` (delta-driven).

## Why `const fn`

The frame table for `SpinnerStyle::Dots` is a 10-element `&'static [&'static
str]`, exposed via:

```rust
impl SpinnerStyle {
    pub const fn frames(self) -> &'static [&'static str] { … }
    pub const fn frame_at(self, tick: Tick) -> &'static str { … }
}
```

The lookup is `O(1)` pointer arithmetic, allocation-free, and usable in
const contexts (e.g., defining compile-time string tables from spinner
glyphs).

Where the const-stable surface stops, we use the safe `Option::unwrap_or`
fallback (`frames.get(idx).copied().unwrap_or("")`) — see the comments
on [`frame_at`](https://docs.rs/cornichon/latest/cornichon/struct.SpinnerStyle.html#method.frame_at)
for the rationale (it's not const-able on stable Rust 1.98 due to
`Index`/`PartialEq` not yet being stable const traits).
