//! Form-widget domain types: [`TextInput`], [`Checkbox`], [`RadioGroup`].
//!
//! These are pure value objects — they own their state, expose query /
//! mutation methods, and have no I/O. Per `AGENTS.md` the domain layer
//! is the innermost ring; it depends on nothing in `application/` or
//! `adapters/`.

// ---------------------------------------------------------------------------
// TextInput
// ---------------------------------------------------------------------------

/// A single-line text input with a cursor.
///
/// The cursor is tracked as a *byte* offset into [`value`](Self::value) —
/// not a grapheme-cluster index. For ASCII text the two are identical;
/// for non-ASCII the byte position is sufficient because
/// [`TextInput::insert_char`] and friends reject invalid byte offsets
/// rather than panicking. The application layer's `update` is
/// responsible for clamping to a valid char boundary before each
/// mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInput {
    /// Current text content (UTF-8 string, may be empty).
    pub value: String,
    /// Cursor position, in bytes. Always `0..=value.len()` and always
    /// aligned to a UTF-8 char boundary.
    pub cursor: usize,
    /// Optional placeholder shown when `value` is empty.
    pub placeholder: Option<String>,
}

impl TextInput {
    /// Construct a new text input with the given placeholder (if any).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            placeholder: None,
        }
    }

    /// Construct a new text input pre-populated with `value` and the
    /// cursor at the end of the string.
    #[must_use]
    pub fn with_value(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.len();
        Self {
            value,
            cursor,
            placeholder: None,
        }
    }

    /// Builder: set the placeholder text.
    #[must_use]
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }

    /// Length in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.value.len()
    }

    /// Whether the value is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Insert a character (or string) at the current cursor position and
    /// advance the cursor past the inserted text.
    ///
    /// Does nothing if `c` would split a UTF-8 codepoint at the
    /// current cursor position.
    pub fn insert(&mut self, c: &str) {
        if !self.value.is_char_boundary(self.cursor) {
            return;
        }
        self.value.insert_str(self.cursor, c);
        self.cursor += c.len();
    }

    /// Remove the character immediately before the cursor (Backspace).
    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        // Find the previous char boundary.
        let prev = self.value[..self.cursor]
            .char_indices()
            .next_back()
            .map_or(0, |(i, _)| i);
        self.value.replace_range(prev..self.cursor, "");
        self.cursor = prev;
    }

    /// Remove the character at the cursor (Delete).
    pub fn delete(&mut self) {
        if self.cursor >= self.value.len() {
            return;
        }
        if !self.value.is_char_boundary(self.cursor) {
            return;
        }
        let next = self.value[self.cursor..]
            .chars()
            .next()
            .map_or(self.value.len(), |c| self.cursor + c.len_utf8());
        self.value.replace_range(self.cursor..next, "");
    }

    /// Move the cursor one character to the left, if possible.
    pub fn move_left(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let prev = self.value[..self.cursor]
            .char_indices()
            .last()
            .map_or(0, |(i, _)| i);
        self.cursor = prev;
    }

    /// Move the cursor one character to the right, if possible.
    pub fn move_right(&mut self) {
        if self.cursor >= self.value.len() {
            return;
        }
        if !self.value.is_char_boundary(self.cursor) {
            return;
        }
        let next = self.value[self.cursor..]
            .chars()
            .next()
            .map_or(self.value.len(), |c| self.cursor + c.len_utf8());
        self.cursor = next;
    }

    /// Move the cursor to the start of the buffer.
    pub const fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// Move the cursor to the end of the buffer.
    pub const fn move_end(&mut self) {
        self.cursor = self.value.len();
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

/// A boolean toggle widget.
///
/// `label` is rendered next to the toggle. `checked` is the current
/// state. Use [`Checkbox::toggle`] to flip, or [`Checkbox::set`] to
/// assign directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkbox {
    /// Whether the box is currently checked.
    pub checked: bool,
    /// Human-readable label rendered next to the box.
    pub label: String,
}

impl Checkbox {
    /// Construct a new checkbox.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            checked: false,
            label: label.into(),
        }
    }

    /// Construct a new checkbox pre-set to `checked`.
    #[must_use]
    pub fn with_checked(label: impl Into<String>, checked: bool) -> Self {
        Self {
            checked,
            label: label.into(),
        }
    }

    /// Set the checked state.
    pub const fn set(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Flip the checked state.
    pub const fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

/// A group of options where exactly one is selected.
///
/// `options` is the list of human-readable labels. The selected index
/// is the position of the currently-checked option. `selected` is
/// always `0..options.len()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroup {
    /// Human-readable option labels.
    pub options: Vec<String>,
    /// Index of the currently-selected option.
    pub selected: usize,
}

impl RadioGroup {
    /// Construct a new radio group from the given option labels.
    /// Panics-free on empty input (the empty group is rendered as a
    /// blank area).
    #[must_use]
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let options: Vec<String> = options.into_iter().map(Into::into).collect();
        Self {
            options,
            selected: 0,
        }
    }

    /// Number of options in the group.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.options.len()
    }

    /// Whether the group has no options.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// The currently-selected option's index.
    #[must_use]
    pub const fn selected(&self) -> usize {
        self.selected
    }

    /// The currently-selected option's label.
    #[must_use]
    pub fn selected_label(&self) -> Option<&str> {
        self.options.get(self.selected).map(String::as_str)
    }

    /// Move the selection to the previous option (wraps at the top).
    pub const fn prev(&mut self) {
        if self.options.is_empty() {
            return;
        }
        self.selected = if self.selected == 0 {
            self.options.len() - 1
        } else {
            self.selected - 1
        };
    }

    /// Move the selection to the next option (wraps at the bottom).
    pub const fn next(&mut self) {
        if self.options.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.options.len();
    }

    /// Set the selected index directly. Clamps to `0..options.len()`.
    pub fn select(&mut self, idx: usize) {
        if self.options.is_empty() {
            return;
        }
        self.selected = idx.min(self.options.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textinput_insert_backspace() {
        let mut ti = TextInput::new();
        ti.insert("h");
        ti.insert("i");
        assert_eq!(ti.value, "hi");
        assert_eq!(ti.cursor, 2);
        ti.backspace();
        assert_eq!(ti.value, "h");
        assert_eq!(ti.cursor, 1);
    }

    #[test]
    fn textinput_move_end_moves_to_len() {
        let mut ti = TextInput::with_value("hi");
        ti.cursor = 0;
        ti.move_end();
        assert_eq!(ti.cursor, 2);
    }

    #[test]
    fn textinput_delete() {
        let mut ti = TextInput::with_value("hi");
        ti.cursor = 0;
        ti.delete();
        assert_eq!(ti.value, "i");
    }

    #[test]
    fn checkbox_toggle() {
        let mut c = Checkbox::new("foo");
        assert!(!c.checked);
        c.toggle();
        assert!(c.checked);
        c.toggle();
        assert!(!c.checked);
    }

    #[test]
    fn radiogroup_next_wraps() {
        let mut rg = RadioGroup::new(["a", "b", "c"]);
        rg.select(2);
        rg.next();
        assert_eq!(rg.selected(), 0);
    }

    #[test]
    fn radiogroup_prev_wraps() {
        let mut rg = RadioGroup::new(["a", "b", "c"]);
        rg.select(0);
        rg.prev();
        assert_eq!(rg.selected(), 2);
    }
}
