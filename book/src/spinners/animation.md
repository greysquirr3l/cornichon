# Spinners · Animation timing

cornichon exposes **two** complementary animation drivers:

| API                                                                                                           | Form     | When to use                                                 |
| ------------------------------------------------------------------------------------------------------------- | -------- | ----------------------------------------------------------- |
| [`tick_at(total: Duration)`](https://docs.rs/cornichon/latest/cornichon/struct.Spinner.html#method.tick_at)   | Absolute | Caller owns a wall clock; calls from a UI event loop        |
| [`advance(elapsed: Duration)`](https://docs.rs/cornichon/latest/cornichon/struct.Spinner.html#method.advance) | Delta    | Caller integrates with a frame `Duration` (game loop, etc.) |

Both produce the same animation; they differ in **how the caller hands
time to the spinner**.

## `advance()` vs `tick_at()`

```rust
// tick_at: clock-driven, deterministic
let total = start.elapsed();
spinner.tick_at(total);  // tick = total / cadence

// advance: delta-driven, "I gave you this much time, figure it out"
spinner.advance(Duration::from_millis(80));  // tick += frames_elapsed
```

`tick_at` computes the **absolute** tick from total uptime. Each call
overwrites the tick; no state accumulates. This is the recommended
driver for **clock-driven animation** (UI spinners, splash screens,
progress meters) — the renderer can call it at any rate, at any
frequency, and the spinner shows the right glyph for the current clock
second.

`advance` is for when the caller manages time as a `Duration` rather than
as an absolute clock (frame loops in games, animation tickers that
integrate with a timer event, etc.). Each call moves the spinner forward
by up to `u8::MAX` frames.

> **Why both?** Every animation library needs the delta form for game
> loops, but UI work — what cornichon is for — wants absolute time.
> Constraining to just one form means the other use case has to reimplement
> the math themselves.

## Driving from `crossterm`

The standard pattern with `crossterm::event`:

```rust,no_run
use std::time::Instant;

let start = Instant::now();
loop {
    let total = start.elapsed();

    for s in spinners.iter_mut() {
        s.tick_at(total);
    }

    terminal.draw(|f| { /* render everything */ })?;

    if event::poll(Duration::from_millis(33))? {
        // handle Esc / q / etc.
    }
}
```

This is exactly what [`cornichon-spinner`](../examples/cornichon-spinner)
does — see its `run` function in `examples/cornichon-spinner/src/main.rs`.

### Why `tick_at`, not `advance`, here

`event::poll(33ms)` blocks ~16-100ms per iteration. With NORMAL cadence
(80ms), `advance(elapsed)` would see ~16-100ms per call — most of which
is **shorter** than every `Cadence` value. `advance` returns `0`, the
tick never advances, the spinner freezes on frame 0.

`tick_at(total)` resets the tick to `total / cadence_millis()`, so even
a 5ms-per-frame loop renders the right glyph for the current uptime.

## Driving from a global clock

If you have a single authoritative clock (a frame driver, a game tick,
etc.), call `tick_at` from that clock on every UI refresh:

```rust
fn tick_global_clock(spinners: &mut [Spinner], total: Duration) {
    for s in spinners.iter_mut() {
        s.tick_at(total);
    }
}
```

The animation **frame rate is decoupled from your render rate** —
calling `tick_at` 5 times a second or 60 times a second both produce
the correct frame as long as you pass the same `total`.

## Driving from a frame `Duration`

```rust,no_run
let dt = last.elapsed();
last = Instant::now();

for s in spinners.iter_mut() {
    s.advance(dt);
}
```

This is the right pattern when your render loop already has a `Duration`
delta (`Instant::now() - last`). `advance` accumulates ticks across
calls — passing a 160ms delta to a NORMAL-cadence spinner yields `+2`
ticks.

`advance` returns the **number of frames advanced** as a `u8` (saturating
at `u8::MAX` — passing `Duration::from_secs(60)` to a 1ms-cadence
spinner doesn't blow up your state).

## Pitfalls

### ❌ Forgetting the elapsed-time accumulator

```rust,ignore
let frame_start = Instant::now();
loop {
    // draw …

    // WRONG: only the *last* iteration's elapsed is fed in.
    spinner.advance(frame_start.elapsed());
}
```

`advance` measures **per-call** elapsed. Passing the same value every
iteration is wrong — pass the _delta between iterations_, not the
_absolute since startup_.

### ❌ Mixing `advance` and `tick_at` for the same spinner

Pick one. `tick_at` overwrites the tick; `advance` adds to it. If you
alternate, the visible frame will jump.

### ❌ Assuming `advance(elapsed)` is "how long since startup"

It's "how much wall-clock time has passed _since the last call_". For
a startup-relative duration, use `tick_at`.

## When to use which

| Caller pattern                                                    | Use                                 |
| ----------------------------------------------------------------- | ----------------------------------- |
| UI event loop with `crossterm::event::poll`                       | `tick_at`                           |
| Single global clock that ticks many widgets                       | `tick_at`                           |
| Game loop with a `Duration` delta integrator                      | `advance`                           |
| Wrapping another animation library that exposes `Duration` deltas | `advance`                           |
| Tests with deterministic time                                     | `tick_at(Duration::from_millis(N))` |

[`cornichon-spinner`]: https://github.com/greysquirr3l/cornichon/tree/main/examples/cornichon-spinner
