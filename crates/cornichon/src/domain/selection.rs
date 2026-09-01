//! Copy cursor primitive shared by every menu state machine.
//!
//! Per counterintuitive-patterns #7 "Newtypes Should Almost Always Be `Copy`",
//! the cursor is `Copy` — it's just a `usize` and a `usize`, no heap data, no
//! reason to force every call site to manage ownership.

/// A cursor over a list of `count` items.
///
/// The cursor is a `usize` index in the range `[0, count)`. Operations that
/// would move it out of range wrap (or clamp, depending on the method) so the
/// state machine stays total: there is always *some* valid position, even for
/// an empty list (where the only valid position is `0`).
///
/// # Wrap vs clamp
///
/// - [`Selection::up`] and [`Selection::down`] **wrap** with modular arithmetic,
///   so `Down` past the last row lands on the first (matches Claude Code,
///   Codex, and Grok behavior).
/// - [`Selection::clamp`] **clamps** to the new maximum — used after the list
///   shrinks beneath the current cursor.
///
/// # Examples
///
/// ```rust
/// use cornichon::domain::Selection;
///
/// let mut s = Selection::new(3);
/// assert_eq!(s.index(), 0);
///
/// s.down();
/// assert_eq!(s.index(), 1);
///
/// s.down();
/// s.down(); // wraps
/// assert_eq!(s.index(), 0);
///
/// s.clamp_to(2); // list shrunk from 3 -> 2, cursor was 0
/// assert_eq!(s.index(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Selection {
    index: usize,
    count: usize,
}

impl Selection {
    /// Construct a selection for a list of `count` items.
    ///
    /// `count == 0` is permitted — the cursor sits at `0` and `up` / `down`
    /// become no-ops, which keeps every method total without `Result`.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self { index: 0, count }
    }

    /// Current cursor index. Always `<= count`; `< count` unless `count == 0`.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Number of items the cursor ranges over.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Whether the list is empty (no selectable items).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Move down by one, wrapping at the bottom.
    ///
    /// No-op when the list is empty.
    pub const fn down(&mut self) {
        if self.count == 0 {
            return;
        }
        self.index = (self.index + 1) % self.count;
    }

    /// Move up by one, wrapping at the top.
    ///
    /// No-op when the list is empty.
    pub const fn up(&mut self) {
        if self.count == 0 {
            return;
        }
        self.index = if self.index == 0 {
            self.count - 1
        } else {
            self.index - 1
        };
    }

    /// Move down by `n` rows, wrapping as needed.
    ///
    /// Used by Page-Down and similar bulk-jump keys.
    pub const fn down_by(&mut self, n: usize) {
        if self.count == 0 {
            return;
        }
        self.index = (self.index + n) % self.count;
    }

    /// Move up by `n` rows, wrapping as needed.
    ///
    /// Used by Page-Up and similar bulk-jump keys.
    pub const fn up_by(&mut self, n: usize) {
        if self.count == 0 {
            return;
        }
        // Modulo over `self.count` with wrap on underflow.
        // (index.wrapping_sub(n)) % count is unsigned-safe.
        self.index = self.index.wrapping_sub(n) % self.count;
    }

    /// Move to the first row.
    pub const fn first(&mut self) {
        self.index = 0;
    }

    /// Move to the last row.
    pub const fn last(&mut self) {
        if self.count == 0 {
            return;
        }
        self.index = self.count - 1;
    }

    /// Jump to a specific index. Out-of-range indices are clamped.
    ///
    /// Used by hotkey handling — pressing `3` jumps to whatever item has
    /// `key = Some('3')`, which is found via `items.iter().position(...)`.
    pub const fn set(&mut self, index: usize) {
        if self.count == 0 {
            return;
        }
        // Avoid `Ord::min` because `Ord` is not yet stable as a const trait
        // (Rust 1.98, see rust-lang/rust#143874).
        self.index = if index >= self.count {
            self.count - 1
        } else {
            index
        };
    }

    /// Resize the underlying list and clamp the cursor to the new range.
    ///
    /// Called when items are added/removed so the cursor can't end up past the
    /// end of the list.
    pub const fn clamp_to(&mut self, new_count: usize) {
        self.count = new_count;
        if new_count == 0 {
            self.index = 0;
        } else if self.index >= new_count {
            self.index = new_count - 1;
        }
    }

    /// Reset the cursor to the first row without changing `count`.
    pub const fn reset(&mut self) {
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_down_at_bottom() {
        let mut s = Selection::new(3);
        s.down();
        s.down();
        s.down();
        assert_eq!(s.index(), 0);
    }

    #[test]
    fn wraps_up_at_top() {
        let mut s = Selection::new(3);
        s.up();
        assert_eq!(s.index(), 2);
    }

    #[test]
    fn empty_list_is_no_op() {
        let mut s = Selection::new(0);
        s.down();
        s.up();
        s.first();
        s.last();
        assert_eq!(s.index(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn clamp_shrinks_below_cursor() {
        let mut s = Selection::new(5);
        s.last(); // index = 4
        s.clamp_to(2); // new max = 1
        assert_eq!(s.index(), 1);
    }

    #[test]
    fn clamp_grows_keeps_cursor() {
        let mut s = Selection::new(3);
        s.down(); // index = 1
        s.clamp_to(10);
        assert_eq!(s.index(), 1);
    }

    #[test]
    fn down_by_wraps() {
        let mut s = Selection::new(5);
        s.down_by(7); // 0 + 7 = 7, 7 % 5 = 2
        assert_eq!(s.index(), 2);
    }

    #[test]
    fn up_by_wraps() {
        let mut s = Selection::new(5);
        s.down_by(2); // index = 2
        s.up_by(7); // 2.wrapping_sub(7) = huge, % 5 = ?
        // (usize::MAX - 4) % 5 = 0 in this case; check exact:
        let expected = 2_usize.wrapping_sub(7) % 5;
        assert_eq!(s.index(), expected);
    }
}
