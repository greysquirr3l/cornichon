//! [`Spinner`] — the application-layer state machine that drives a
//! [`SpinnerStyle`](crate::domain::SpinnerStyle) animation.
//!
//! Unlike the menu widgets (which are keyboard-driven), [`Spinner`] is
//! clock-driven: callers advance the animation by feeding in the elapsed
//! `Duration` since the last update. This keeps the library pure — no
//! `Instant::now()`, no internal threads — and makes spinners deterministic
//! in tests (just pass a fixed duration).
//!
//! # Examples
//!
//! ```rust
//! use cornichon::prelude::*;
//! use core::time::Duration;
//!
//! let mut s = Spinner::builder()
//!     .style(SpinnerStyle::Dots)
//!     .label("Loading")
//!     .build();
//!
//! s.advance(Duration::from_millis(80));
//! assert_eq!(s.current_frame(), SpinnerStyle::Dots.frames()[1]);
//! ```
//!
//! # Driving from `crossterm`
//!
//! ```rust,ignore
//! let mut last = std::time::Instant::now();
//! loop {
//!     let now = std::time::Instant::now();
//!     spinner.advance(now.duration_since(last));
//!     last = now;
//!     terminal.draw(|f| f.render_widget(&spinner, f.area()))?;
//!     // ... sleep ~cadence, read input, etc.
//! }
//! ```

use core::time::Duration;

use crate::domain::{Cadence, SpinnerStyle, Theme, Tick};

/// A spinner state machine.
///
/// Composed of:
/// - a [`SpinnerStyle`] (which animation to draw)
/// - an optional label (rendered dim, after the frame glyph)
/// - a [`Cadence`] (how often to advance the frame)
/// - a [`Tick`] counter (which frame to draw)
/// - an optional `custom_frames` override (replaces the style's frame
///   table entirely — useful for app-specific animations)
///
/// Constructed via [`Spinner::builder`]; defaults to [`SpinnerStyle::Dots`]
/// with a `Some(80ms)` cadence and no label.
#[derive(Debug, Clone)]
pub struct Spinner {
    style: SpinnerStyle,
    label: Option<String>,
    cadence: Cadence,
    tick: Tick,
    theme: Theme,
    /// Optional override frames. When `Some`, [`current_frame`](Self::current_frame)
    /// and [`frame_count`](Self::frame_count) read from this slice instead of
    /// [`SpinnerStyle::frames`]. Public callers should reach this via
    /// [`Spinner::with_frames`] or [`SpinnerBuilder::frames`], not by direct
    /// assignment.
    custom_frames: Option<&'static [&'static str]>,
}

impl Spinner {
    /// Start a builder with all fields at their defaults.
    #[must_use]
    pub fn builder() -> SpinnerBuilder {
        SpinnerBuilder::default()
    }

    /// The current animation style.
    #[must_use]
    pub const fn style(&self) -> SpinnerStyle {
        self.style
    }

    /// The label rendered after the frame glyph, if any.
    #[must_use]
    pub const fn label(&self) -> Option<&str> {
        // `Option::as_deref` is not yet stable as a const fn; explicit match.
        match &self.label {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    /// The frame cadence.
    #[must_use]
    pub const fn cadence(&self) -> Cadence {
        self.cadence
    }

    /// The current tick counter.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// The active theme.
    #[must_use]
    pub const fn theme(&self) -> Theme {
        self.theme
    }

    /// The frame glyph for the current tick.
    ///
    /// Equivalent to `self.style().frame_at(self.tick())` when no
    /// custom frames override is set; otherwise reads from the override
    /// slice. Returns `""` if the override is somehow empty (defensive).
    #[must_use]
    pub fn current_frame(&self) -> &'static str {
        if let Some(frames) = self.custom_frames {
            let len = frames.len();
            if len == 0 {
                return "";
            }
            let idx = (self.tick.0 as usize) % len;
            frames.get(idx).copied().unwrap_or("")
        } else {
            self.style.frame_at(self.tick)
        }
    }

    /// Number of frames in this spinner's animation.
    ///
    /// Reads from the custom-frames override when set, otherwise from
    /// the underlying [`SpinnerStyle::frame_count`].
    #[must_use]
    pub const fn frame_count(&self) -> usize {
        if let Some(frames) = self.custom_frames {
            frames.len()
        } else {
            self.style.frame_count()
        }
    }

    /// Build a spinner with custom frame glyphs, bypassing the bundled
    /// [`SpinnerStyle`] catalog. Convenience for
    /// `Spinner::builder().frames(frames).build()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cornichon::prelude::*;
    ///
    /// const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸"];
    /// let s = Spinner::with_frames(SPINNER);
    /// assert_eq!(s.frame_count(), 4);
    /// assert_eq!(s.current_frame(), "⠋");
    /// ```
    #[must_use]
    pub fn with_frames(frames: &'static [&'static str]) -> Self {
        Self::builder().frames(frames).build()
    }

    /// Advance the animation by `elapsed`.
    ///
    /// If `elapsed >= cadence.millis()`, the tick counter is incremented by
    /// the number of frame intervals that fit (typically `1`). Returns the
    /// number of frames advanced — `0` when the elapsed duration was
    /// shorter than `cadence.millis()`, `>= 1` otherwise.
    ///
    /// Saturates: passing a multi-second elapsed time advances the tick by
    /// at most `u8::MAX` frames per call to keep the math bounded.
    ///
    /// **Note:** `advance` measures deltas against `cadence.millis()`, so in
    /// loops where per-iteration elapsed is sub-cadence (e.g., a 60fps
    /// render loop with `Cadence::NORMAL` of 80ms), `advance` will return
    /// `0` most frames. For clock-driven animation, prefer
    /// [`tick_at`](Self::tick_at).
    pub fn advance(&mut self, elapsed: Duration) -> u8 {
        let elapsed_ms = u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX);
        let cadence_ms = u64::from(self.cadence.millis());
        if elapsed_ms < cadence_ms {
            return 0;
        }
        let frames = u8::try_from(elapsed_ms / cadence_ms).unwrap_or(u8::MAX);
        for _ in 0..frames {
            self.tick = self.tick.next();
        }
        frames
    }

    /// Set the tick counter to match `total_elapsed` since startup.
    ///
    /// `tick_at` computes the tick as `total_elapsed / cadence.millis()`
    /// (saturating on overflow), so each spinner animates at exactly its
    /// configured pace — independent of the renderer's frame rate.
    ///
    /// This is the recommended driver for clock-based animation
    /// (animation UIs, splash screens, progress indicators): the caller
    /// tracks total uptime and calls `tick_at` each frame, and the
    /// spinner does the right thing regardless of how fast the caller
    /// calls back.
    ///
    /// ```rust,ignore
    /// let start = Instant::now();
    /// loop {
    ///     let total = start.elapsed();
    ///     for s in spinners.iter_mut() {
    ///         s.tick_at(total);
    ///     }
    ///     terminal.draw(...)?;
    /// }
    /// ```
    pub fn tick_at(&mut self, total_elapsed: Duration) {
        let cadence_ms = u128::from(self.cadence.millis());
        let tick_n = u32::try_from(total_elapsed.as_millis() / cadence_ms).unwrap_or(u32::MAX);
        self.tick = Tick(tick_n);
    }

    /// Switch to a different style, resetting the tick counter so the new
    /// animation starts from frame zero.
    pub fn set_style(&mut self, style: SpinnerStyle) {
        if self.style != style {
            self.style = style;
            self.tick = Tick::ZERO;
        }
    }

    /// Replace the label.
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }
}

/// Builder for [`Spinner`].
///
/// Constructed via [`Spinner::builder`]. Required: nothing — every field has
/// a default. Optional: [`style`](SpinnerBuilder::style),
/// [`label`](SpinnerBuilder::label), [`cadence`](SpinnerBuilder::cadence),
/// [`theme`](SpinnerBuilder::theme), [`frames`](SpinnerBuilder::frames).
#[derive(Debug, Default, Clone)]
pub struct SpinnerBuilder {
    style: Option<SpinnerStyle>,
    label: Option<String>,
    cadence: Option<Cadence>,
    theme: Theme,
    /// Custom frame table; takes precedence over `style` when set.
    frames: Option<&'static [&'static str]>,
}

impl SpinnerBuilder {
    /// Set the animation style.
    #[must_use]
    pub const fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Set the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the cadence.
    #[must_use]
    pub const fn cadence(mut self, cadence: Cadence) -> Self {
        self.cadence = Some(cadence);
        self
    }

    /// Set the theme.
    #[must_use]
    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Build the spinner.
    #[must_use]
    pub fn build(self) -> Spinner {
        Spinner {
            style: self.style.unwrap_or_default(),
            label: self.label,
            cadence: self.cadence.unwrap_or_default(),
            tick: Tick::ZERO,
            theme: self.theme,
            custom_frames: self.frames,
        }
    }

    /// Provide a custom frame table that takes precedence over
    /// [`style`](Self::style). The slice must outlive the spinner —
    /// pass a `const` static or `&'static` literal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cornichon::prelude::*;
    ///
    /// const FRAMES: &[&str] = &["⠋", "⠙", "⠹"];
    /// let spinner = Spinner::builder().frames(FRAMES).build();
    /// assert_eq!(spinner.frame_count(), 3);
    /// ```
    #[must_use]
    pub const fn frames(mut self, frames: &'static [&'static str]) -> Self {
        self.frames = Some(frames);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults_are_safe() {
        let s = Spinner::builder().build();
        assert_eq!(s.style(), SpinnerStyle::Dots);
        assert_eq!(s.label(), None);
        assert_eq!(s.cadence(), Cadence::NORMAL);
        assert_eq!(s.tick(), Tick::ZERO);
        assert_eq!(s.current_frame(), SpinnerStyle::Dots.frame_at(Tick::ZERO));
    }

    #[test]
    fn advance_below_cadence_is_zero() {
        let mut s = Spinner::builder().cadence(Cadence(100)).build();
        let n = s.advance(Duration::from_millis(50));
        assert_eq!(n, 0);
        assert_eq!(s.tick(), Tick::ZERO);
    }

    #[test]
    fn advance_at_one_interval_advances_once() {
        let mut s = Spinner::builder().cadence(Cadence(100)).build();
        let n = s.advance(Duration::from_millis(100));
        assert_eq!(n, 1);
        assert_eq!(s.tick(), Tick(1));
    }

    #[test]
    fn advance_by_multiple_intervals() {
        let mut s = Spinner::builder().cadence(Cadence(100)).build();
        let n = s.advance(Duration::from_millis(450));
        assert_eq!(n, 4); // 450 / 100 = 4.5, floor = 4
        assert_eq!(s.tick(), Tick(4));
    }

    #[test]
    fn advance_saturates_at_u8_max() {
        let mut s = Spinner::builder().cadence(Cadence(1)).build();
        let n = s.advance(Duration::from_mins(1)); // 60_000 / 1 = 60_000, way more than u8::MAX
        assert_eq!(n, u8::MAX);
    }

    #[test]
    fn tick_at_drives_from_total_elapsed() {
        let mut s = Spinner::builder().cadence(Cadence(50)).build();
        // Total elapsed 250ms with 50ms cadence → tick = 250/50 = 5.
        s.tick_at(Duration::from_millis(250));
        assert_eq!(s.tick(), Tick(5));
    }

    #[test]
    fn tick_at_is_independent_of_loop_rate() {
        // Same total elapsed, same cadence → same tick, no matter how
        // many `tick_at` calls happen in between.
        let mut a = Spinner::builder().cadence(Cadence(60)).build();
        let mut b = Spinner::builder().cadence(Cadence(60)).build();
        a.tick_at(Duration::from_millis(180));
        for _ in 0..5 {
            // simulate 60fps loop; each "tick_at" at sub-cadence should
            // not advance further because we set the absolute tick.
            b.tick_at(Duration::from_millis(180));
        }
        assert_eq!(a.tick(), b.tick());
    }

    #[test]
    fn set_style_resets_tick_when_changed() {
        let mut s = Spinner::builder().style(SpinnerStyle::Dots).build();
        s.advance(Duration::from_millis(80 * 5));
        assert_eq!(s.tick(), Tick(5));

        s.set_style(SpinnerStyle::Line);
        assert_eq!(s.tick(), Tick::ZERO);
        assert_eq!(s.style(), SpinnerStyle::Line);
    }

    #[test]
    fn set_style_same_value_keeps_tick() {
        let mut s = Spinner::builder().style(SpinnerStyle::Dots).build();
        s.advance(Duration::from_millis(80 * 3));
        let before = s.tick();
        s.set_style(SpinnerStyle::Dots); // same style, no reset
        assert_eq!(s.tick(), before);
    }
}
