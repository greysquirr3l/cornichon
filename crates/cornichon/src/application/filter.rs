//! Filter helper for slash menus.
//!
//! Matches items whose `name` (with any leading `/` stripped) starts with
//! `query`, case-insensitively. Returns the *original* indices of matching
//! items — [`SlashMenu`](crate::application::menu::SlashMenu) uses those to
//! map a filtered-row selection back to the underlying `MenuItem`.

use crate::domain::MenuItem;

/// Return the indices of `items` whose `name` (with any leading `/`
/// stripped) starts with `query`, case-insensitively.
///
/// An empty `query` matches every item (preserves order).
///
/// # Allocation
///
/// Allocates zero heap memory per item; the `eq_ignore_ascii_case` call on
/// a prefix slice is allocation-free for ASCII (and the menu library is
/// primarily ASCII in practice). Non-ASCII names fall back to byte-level
/// comparison via `to_ascii_lowercase` to keep the implementation simple;
/// if you need full Unicode case-folding, lift the loop into the caller.
///
/// # Examples
///
/// ```rust
/// use cornichon::application::filter_indices;
/// use cornichon::domain::MenuItem;
///
/// let items = [
///     MenuItem::try_from(("/agents", "desc"))?,
///     MenuItem::try_from(("/clear", "desc"))?,
/// ];
///
/// // Empty query matches everything.
/// assert_eq!(filter_indices(&items, ""), vec![0, 1]);
///
/// // Prefix match (case-insensitive, leading `/` stripped from item names).
/// assert_eq!(filter_indices(&items, "ag"), vec![0]);
/// # Ok::<(), cornichon::domain::Error>(())
/// ```
#[must_use]
pub fn filter_indices(items: &[MenuItem], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..items.len()).collect();
    }
    items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            let stripped = item.name.strip_prefix('/').unwrap_or(item.name.as_str());
            // No indexing-slicing — use `.get(..n)` for the prefix.
            if stripped.len() < query.len() {
                return None;
            }
            match stripped.get(..query.len()) {
                Some(prefix) if prefix.eq_ignore_ascii_case(query) => Some(i),
                _ => None,
            }
        })
        .collect()
}
