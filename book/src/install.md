# Install

Add cornichon to your `Cargo.toml`:

```toml
[dependencies]
cornichon = "0.1"
ratatui = "0.30"
crossterm = "0.29"
```

That's it. cornichon re-exports the `ratatui` / `crossterm` types it touches
(`KeyEvent`, `Buffer`, `Rect`, `Widget`) under [`cornichon::ports`] so you
don't need to write a separate `use ratatui::…` import for the common cases —
but if you'd rather name ratatui directly in your own code, both work.

## Feature flags

cornichon has no feature flags today. Future additions (themed bundles, async
runtime support, etc.) will land as opt-in features rather than default-on
behavior.

## Versioning

cornichon follows [Semantic Versioning]. 0.x releases are allowed to break
the API between minor versions until 1.0 is cut. Once 1.0 ships, the
contract is:

- Patch versions: bug fixes only; no API changes.
- Minor versions: additive changes only; no breakage.
- Major versions: anything goes, but deprecations will be marked.

[Semantic Versioning]: https://semver.org/
[`cornichon::ports`]: https://docs.rs/cornichon/latest/cornichon/ports/index.html
