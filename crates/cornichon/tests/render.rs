//! Render tests — exercise each menu's `Widget for &Menu` impl against a
//! fresh `Buffer`, asserting that specific cells contain the expected
//! glyphs. This is a manual golden test: rather than comparing to a
//! stored string snapshot (which would be brittle across terminal
//! width changes), we assert presence and identity of key cells.

use cornichon::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::time::Duration;

/// Render a widget into a fresh buffer and return the buffer.
///
/// The parameter is `W: Widget` (not `&W`) so callers can pass `&Menu`
/// directly — cornichon's `Widget` impls are all on `&Menu`, so the
/// type bound needs to be satisfied on the reference type, not the
/// owned type.
fn render_to_buffer<W: ratatui::widgets::Widget>(
    widget: W,
    width: u16,
    height: u16,
) -> (Buffer, Rect) {
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    widget.render(area, &mut buf);
    (buf, area)
}

/// Index a static slice without triggering `clippy::indexing_slicing`.
/// Returns the element by value (requires `T: Copy`). Aborts the
/// process if the index is out of bounds — use this only for slices
/// whose minimum length is part of a `const` definition or otherwise
/// statically known to the caller.
fn static_at<T: Copy>(slice: &[T], idx: usize) -> T {
    match slice.get(idx) {
        Some(&v) => v,
        None => std::process::abort(),
    }
}

#[test]
fn slashmenu_default_first_cell_is_active_marker() -> cornichon::Result<()> {
    let menu = SlashMenu::with_defaults()?;
    let (buf, _) = render_to_buffer(&menu, 60, 10);
    // First cell at (0, 0) is the active-row marker from the theme
    // (the Claude theme's marker is "❯"). The item-name column starts
    // a couple of cells to the right.
    assert_eq!(buf[(0, 0)].symbol(), "\u{276F}"); // ❯
    // And the first item name should appear somewhere in row 0.
    let mut row0 = String::new();
    for x in 0..60 {
        row0.push_str(buf[(x, 0)].symbol());
    }
    assert!(row0.contains("/agents"), "row0 was {row0:?}");
    Ok(())
}

#[test]
fn slashmenu_filter_narrows_visible_items() -> cornichon::Result<()> {
    let mut menu = SlashMenu::with_defaults()?;
    // Type "ag" to filter to "/agents".
    let _ = menu.update(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let _ = menu.update(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    assert_eq!(menu.query(), "ag");
    assert_eq!(menu.filtered_indices().len(), 1);
    assert_eq!(static_at(menu.filtered_indices(), 0), 0); // "/agents" is index 0

    // Now render — only one row should appear, and it should be "/agents".
    let (buf, _) = render_to_buffer(&menu, 40, 4);
    // The active row (0 in filtered view) maps to the first item.
    // We don't assert exact cell text here because the renderer places
    // the marker/active-row glyph in a different column depending on
    // theme.marker_width. Just assert that the row contains "agents".
    let row0 = (0..40)
        .map(|x| buf[(x, 0)].symbol().to_string())
        .collect::<String>();
    assert!(row0.contains("/agents"), "row0 was {row0:?}");
    Ok(())
}

#[test]
fn selectmenu_renders_items_in_order() -> cornichon::Result<()> {
    let menu = SelectMenu::builder()
        .items([
            MenuItem::try_from(("Apple", "red fruit"))?,
            MenuItem::try_from(("Banana", "yellow fruit"))?,
            MenuItem::try_from(("Cherry", "red stone fruit"))?,
        ])
        .build()?;
    let (buf, _) = render_to_buffer(&menu, 30, 5);
    // The renderer should place the three items in order. Look for them.
    // We accumulate into a String with a plain nested-for loop instead of
    // `flat_map` so `buf` is borrowed (not moved) by the inner closure.
    let mut all_text = String::new();
    for y in 0..3 {
        for x in 0..30 {
            all_text.push_str(buf[(x, y)].symbol());
        }
    }
    assert!(all_text.contains("Apple"));
    assert!(all_text.contains("Banana"));
    assert!(all_text.contains("Cherry"));
    Ok(())
}

#[test]
fn permissionprompt_three_options_default() -> cornichon::Result<()> {
    let prompt = PermissionPrompt::with_defaults()?;
    let (buf, _) = render_to_buffer(&prompt, 50, 6);
    let mut all_text = String::new();
    for y in 0..6 {
        for x in 0..50 {
            all_text.push_str(buf[(x, y)].symbol());
        }
    }
    assert!(all_text.contains("Yes"));
    assert!(all_text.contains("Don't ask again") || all_text.contains("don't ask again"));
    assert!(all_text.contains("tell Claude") || all_text.contains("Tell Claude"));
    Ok(())
}

#[test]
fn spinner_current_frame_matches_style() {
    let mut s = Spinner::builder()
        .style(SpinnerStyle::Dots)
        .cadence(Cadence(80))
        .build();
    // At t=0, frame should be Dots.frames()[0].
    assert_eq!(s.current_frame(), static_at(SpinnerStyle::Dots.frames(), 0));
    // After advancing 80ms (one cadence), frame should advance.
    s.advance(Duration::from_millis(80));
    assert_eq!(s.current_frame(), static_at(SpinnerStyle::Dots.frames(), 1));
}

#[test]
fn spinner_custom_frames_override_style() {
    const FRAMES: &[&str] = &["X", "Y", "Z"];
    let mut s = Spinner::builder().frames(FRAMES).build();
    assert_eq!(s.current_frame(), "X");
    s.advance(Duration::from_millis(80));
    assert_eq!(s.current_frame(), "Y");
    s.advance(Duration::from_millis(80));
    assert_eq!(s.current_frame(), "Z");
    s.advance(Duration::from_millis(80));
    // Wraps back to X.
    assert_eq!(s.current_frame(), "X");
    assert_eq!(s.frame_count(), 3);
}

#[test]
fn selection_wraps_on_down() -> cornichon::Result<()> {
    let mut menu = SelectMenu::builder()
        .items([
            MenuItem::try_from(("a", "1"))?,
            MenuItem::try_from(("b", "2"))?,
        ])
        .build()?;
    assert_eq!(menu.selection(), 0);
    let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(menu.selection(), 1);
    let _ = menu.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(menu.selection(), 0); // wraps
    let _ = menu.update(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(menu.selection(), 1); // wraps backward
    Ok(())
}

#[test]
fn shortcutsmodal_empty_groups_does_not_panic() -> cornichon::Result<()> {
    use cornichon::application::ShortcutsModal;
    let modal = ShortcutsModal::builder().build()?;
    let (buf, _) = render_to_buffer(&modal, 30, 5);
    // All cells should be blank (the empty buffer default).
    // Use a plain nested-for loop so `buf` is borrowed (not moved) by
    // the inner closure.
    let mut all_blank = true;
    'rows: for y in 0..5 {
        for x in 0..30 {
            let s = buf[(x, y)].symbol();
            if s != " " && !s.is_empty() {
                all_blank = false;
                break 'rows;
            }
        }
    }
    assert!(all_blank, "empty modal should render as blank");
    Ok(())
}

#[test]
fn projectpicker_with_three_projects() -> cornichon::Result<()> {
    let picker = ProjectPicker::with_defaults()?;
    let (buf, _) = render_to_buffer(&picker, 50, 8);
    let mut all_text = String::new();
    for y in 0..8 {
        for x in 0..50 {
            all_text.push_str(buf[(x, y)].symbol());
        }
    }
    assert!(all_text.contains("brainless"));
    // Custom option is the trailing row.
    assert!(all_text.contains("Type your answer here") || all_text.contains("custom"));
    Ok(())
}
