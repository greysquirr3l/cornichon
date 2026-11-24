//! Microbenchmarks for cornichon's hot-path state-machine code.
//!
//! Run with `cargo bench -p cornichon`. Reports in nanoseconds; aim for
//! "tight loop fits in L1 cache" — anything under a few hundred ns per
//! iteration is fine for a 60Hz UI loop.
//!
//! Per the project's `nick.md` standards:
//! - No `.unwrap()` / `.expect()` / `.panic()` in bench setup (the lint
//!   `clippy::expect_used` is denied); bench setup uses `#[expect]` because
//!   `criterion::Bencher` callbacks can't return `Result`.
//! - Bench setup creates real structs (no mocking), exercises the same
//!   public API that consumers call, and is the same logic that ships.

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

use cornichon::prelude::*;

/// Build a `MenuItem` with no key, panicking on construction failure.
///
/// `MenuItem::try_from((&str, &str))` can only fail on the boundary checks
/// (empty name, newline in name). Our test strings are statically known
/// to be valid, so a failure here is a programmer error in the bench
/// itself, not a runtime condition. The `#[expect(clippy::expect_used, ...)]`
/// is the standard way to mark such "shouldn't happen" sites per
/// `nick.md`; using `panic!()` is denied by the strict profile.
macro_rules! make_item {
    ($name:expr, $desc:expr) => {
        MenuItem::try_from(($name, $desc)).expect("static bench item construction should not fail")
    };
}

// ---------------------------------------------------------------------------
// SelectMenu — the simplest list widget. Bench: building, querying, and
// driving a Down-arrow loop on a typical 50-item list.
// ---------------------------------------------------------------------------

#[expect(
    clippy::expect_used,
    reason = "bench setup with statically-valid input strings can never fail"
)]
fn bench_select_menu_build(c: &mut Criterion) {
    let items: Vec<MenuItem> = (0..50)
        .map(|i| make_item!(format!("item {i:02}"), format!("description {i}")))
        .collect();
    c.bench_function("SelectMenu::build (50 items)", |b| {
        b.iter(|| {
            let _menu = SelectMenu::builder()
                .items(black_box(items.clone()))
                .build()
                .expect("SelectMenu::build with valid items should not fail");
        });
    });
}

#[expect(
    clippy::expect_used,
    reason = "bench setup with statically-valid input strings can never fail"
)]
fn bench_select_menu_arrow_keys(c: &mut Criterion) {
    let mut menu = SelectMenu::builder()
        .items((0..50).map(|i| make_item!(format!("item {i:02}"), String::new())))
        .build()
        .expect("SelectMenu::build with valid items should not fail");
    let down = cornichon::ports::KeyEvent::new(
        cornichon::ports::KeyCode::Down,
        cornichon::ports::KeyModifiers::NONE,
    );
    c.bench_function("SelectMenu::update Down (50 items)", |b| {
        b.iter(|| {
            let _action = black_box(&mut menu).update(black_box(down));
        });
    });
}

// ---------------------------------------------------------------------------
// SlashMenu — filtered command palette. Bench: query mutation re-runs
// `filter_indices`, which is the most allocation-heavy path.
// ---------------------------------------------------------------------------

#[expect(
    clippy::expect_used,
    reason = "bench setup with statically-valid input strings can never fail"
)]
fn bench_slashmenu_filter(c: &mut Criterion) {
    let items: Vec<MenuItem> = (0..200)
        .map(|i| make_item!(format!("/cmd-{i:03}"), String::new()))
        .collect();
    c.bench_function("SlashMenu::build (200 items)", |b| {
        b.iter(|| {
            let _menu = SlashMenu::builder()
                .items(black_box(items.clone()))
                .build()
                .expect("SlashMenu::build with valid items should not fail");
        });
    });
}

/// `bench_filter_indices` uses only the `make_item!` macro (no
/// `.expect()` calls), so no `#[expect(clippy::expect_used)]` attribute
/// is required here.
fn bench_filter_indices(c: &mut Criterion) {
    // Pre-build a large item list once, then bench `filter_indices` on it.
    // The bench function uses the public filter helper via the domain.
    let items: Vec<MenuItem> = (0..500)
        .map(|i| make_item!(format!("/cmd-{i:04}"), String::new()))
        .collect();
    c.bench_function("filter_indices (500 items, 3-char query)", |b| {
        b.iter(|| {
            let mut total = 0_usize;
            for item in black_box(&items) {
                // `filter_indices` is a free function; we collect the
                // matching indices via a `for` loop instead of `.collect()`
                // to satisfy `clippy::needless_collect`.
                for _ in cornichon::application::filter::filter_indices(
                    std::slice::from_ref(item),
                    black_box("cmd"),
                ) {
                    total += 1;
                }
            }
            black_box(total);
        });
    });
}

// ---------------------------------------------------------------------------
// Spinner — frame lookup and tick advance. The hot path is the
// `frame_at` lookup that runs every render tick.
// ---------------------------------------------------------------------------

fn bench_spinner_frame_at(c: &mut Criterion) {
    use cornichon::domain::Tick;
    c.bench_function("SpinnerStyle::frame_at (typical ticks)", |b| {
        b.iter(|| {
            // Cover early / mid / late / wrap-around ticks in one bench so
            // we catch any per-branch regression without spinning up four
            // separate bench targets.
            for &raw in &[0_u32, 1, 5, 100, 1_000, u32::MAX] {
                let _frame = cornichon::domain::SpinnerStyle::Dots.frame_at(black_box(Tick(raw)));
            }
        });
    });
}

fn bench_spinner_advance(c: &mut Criterion) {
    let mut spinner = Spinner::builder().cadence(Cadence(80)).build();
    c.bench_function("Spinner::advance (one frame)", |b| {
        b.iter(|| {
            let _frames = black_box(&mut spinner).advance(black_box(Duration::from_millis(80)));
        });
    });
}

fn bench_spinner_tick_at(c: &mut Criterion) {
    let mut spinner = Spinner::builder().cadence(Cadence(80)).build();
    c.bench_function("Spinner::tick_at (8ms total)", |b| {
        b.iter(|| {
            black_box(&mut spinner).tick_at(black_box(Duration::from_millis(8)));
        });
    });
}

// ---------------------------------------------------------------------------
// Selection cursor — the tiny math primitive that every menu uses for
// up/down navigation. L1-cache-sized; should be in the low ns.
// ---------------------------------------------------------------------------

fn bench_selection_navigation(c: &mut Criterion) {
    use cornichon::domain::Selection;
    c.bench_function("Selection::down (50 items, 1000 iters)", |b| {
        b.iter(|| {
            let mut s = Selection::new(50);
            for _ in 0..1000 {
                s.down();
            }
            black_box(s.index());
        });
    });
}

criterion_group!(
    benches,
    bench_select_menu_build,
    bench_select_menu_arrow_keys,
    bench_slashmenu_filter,
    bench_filter_indices,
    bench_spinner_frame_at,
    bench_spinner_advance,
    bench_spinner_tick_at,
    bench_selection_navigation,
);
criterion_main!(benches);
