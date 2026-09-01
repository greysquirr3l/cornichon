# Contributing

cornichon is intentionally small: 5 menus + 1 spinner over the four-layer
`src/{domain,ports,application,adapters}` layout described in
[`AGENTS.md`](AGENTS.md). Adding a widget means adding code to each layer
in the right place — that's the whole workflow.

## Workflow

1. **Decide on the trait split.** A new widget is:
   - **Total**: `update()` returns an enum, never a `Result`.
   - **Self-contained**: the widget owns its state. No external `&mut State`.
   - **Theme-driven**: only colors and the active-row marker come from
     `Theme`; never hardcode hex codes.

   If your widget breaks any of these, that's a design question worth
   raising as an issue first.

2. **Add a domain type.** `crates/cornichon/src/domain/` — pure value
   objects, **no** `ratatui`/`crossterm` imports. Boundary types that wrap
   raw input (e.g., `MenuItem`, `AnnotatedItem`) implement `TryFrom<...>`
   at the boundary, so the rest of the widget can trust invariant-carrying
   types.

3. **Add an `application::menu` (or `application` directly for non-menu
   widgets).** The state machine lives here. Methods:
   - `update(&mut self, event: KeyEvent) -> MenuAction` is total.
   - `builder()` returns a builder with `#[derive(Default)]` where possible.
   - Accessors return `&` borrows or `Copy` values; getters are `const fn`
     where feasible.

4. **Add a `adapters::ratatui` renderer.** `Widget for &Menu` so consumers
   call `frame.render_widget(&menu, area)` without naming ratatui
   types. Render uses `Buffer::set_line(...)` rather than `Paragraph` or
   `List` — keeps the visuals decoupled from ratatui's primitives.

5. **Add tests.** Three layers, all required:
   - Unit tests for state-machine `update()` behavior.
   - Doctests in the public rustdoc.
   - A Buffer render test under `crates/cornichon/tests/render.rs`.

6. **Wire the re-exports.** Add the type to `domain/mod.rs`,
   `application/mod.rs`, `lib.rs`, and `prelude` if it's a common-public
   surface (a widget is; a private helper isn't).

7. **Wire the book.** Add a chapter under
   `book/src/widgets/` and link it from `book/src/SUMMARY.md`.

8. **Verify.** Run `cargo l --workspace --all-targets` and
   `cargo test --workspace`. Both must be green before opening a PR.

## Lint profile

The project enforces a strict clippy profile (see
[`.cargo/config.toml`](.cargo/config.toml) — the `cargo l` alias). The
denied lints are:

| Lint                         | Why                                                     |
| ---------------------------- | ------------------------------------------------------- |
| `clippy::unwrap_used`        | No `.unwrap()` or `.expect()` anywhere — even in tests. |
| `clippy::expect_used`        | Same — `.expect()` is a panic waiting to happen.        |
| `clippy::panic`              | No `panic!` macro; same reason.                         |
| `clippy::indexing_slicing`   | No `&xs[i]` or `&xs[..]`; use `.get(...)`.              |
| `clippy::cast_ptr_alignment` | No safe-looking pointer casts.                          |
| `clippy::suspicious`         | Catch-all for foot-guns.                                |

If you hit a lint that you genuinely can't satisfy, use
`#[expect(clippy::LINT_NAME, reason = "…")]` — _not_ `#[allow]`.
`#[expect]` warns when the suppression is no longer needed; `#[allow]`
sits silent forever.

## What _not_ to do

- **Don't add `#[allow(...)]` suppressions.** Use `#[expect(...)]` with
  a `reason` string. Reviewers will push back on `[allow]` because they
  go silently stale.
- **Don't add new dependencies to cornichon's `Cargo.toml` without
  discussion.** The library currently depends on `ratatui`, `crossterm`,
  and `thiserror` only.
- **Don't add `Result` to `update()`.** Menus are total by design. If
  you need error context, capture it in the `MenuAction` enum.
- **Don't `panic!` for "should never happen" cases.** Use
  `.unwrap_or_else(|_| unreachable!("..."))` — the `unreachable!()` is a
  `panic!` and looks ugly to lints, but it has a single, deliberate
  blast radius. If you find yourself adding more than two of these in a
  single function, that's a design problem.

## Adding a new spinner

Two files:

1. `crates/cornichon/src/domain/spinner.rs`: add a `SpinnerStyle`
   variant, name it in `SpinnerStyle::name(self)`, and add the frame
   table inside `SpinnerStyle::frames(self)` (the matching arm in
   the `match`).
2. Add a test in `mod tests` that walks `SpinnerStyle::all_names()`
   and verifies the variant is present.

There is no other state to maintain — `Spinner` reads the frame table
at render time. To add a custom frame programmatically, use
`Spinner::with_frames(...)` once it's available (v0.2 milestone; not
yet in the public API).

## Running the docs locally

```bash
mdbook serve book       # http://localhost:3000
mdbook build book       # outputs to book/book/
```

If you don't have mdbook yet, `cargo install mdbook --locked`. Or run
the wrapper: `cargo book serve` (uses the workspace-level `tools/mdbook`
binary so you don't need to remember the exact `mdbook` invocation).

## Code review checklist

Reviewers look for:

- [ ] New public types have rustdoc with at least one runnable example.
- [ ] Builders consume `self` and return `Self` (true builder pattern).
- [ ] `update` returns `MenuAction`, never `Result`.
- [ ] Theme customizable via `.theme(theme)` setter.
- [ ] `cargo l --workspace --all-targets` green.
- [ ] `cargo test --workspace` green.
- [ ] Book chapter added or updated.

## Release process

TBD — pinned to 0.x until the API stabilizes. See `AGENTS.md` for the
project-local conventions.
