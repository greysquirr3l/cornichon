//! Visual themes — color palettes matching the brainless captures.
//!
//! Three themes ship as `const`s: [`CLAUDE`], [`CODEX`], [`GROK`]. Custom
//! themes are constructed as struct literals — there is intentionally no
//! runtime builder, because themes should be statically known per
//! application.
//!
//! # Why `Theme` lives in `domain`
//!
//! `Theme` is a pure value object — colors and a marker string, no I/O, no
//! allocation. It uses [`ratatui::style::Color`] as the underlying color
//! representation because, per the pragmatic trait-split decision, ratatui is
//! the renderer (not a varying infrastructure backend). Abstracting Color
//! into a domain-only newtype would buy nothing for a renderer that's 99%
//! ratatui in practice.

use ratatui::style::Color;

/// A visual theme — colors + active-row marker.
///
/// All fields are `Copy` so themes are cheap to pass by value, which matches
/// how often they're threaded through the menu update + render path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    /// Primary text color (selected row label, headers).
    pub fg: Color,
    /// Secondary text color (descriptions, hints).
    pub fg_dim: Color,
    /// Tertiary text color (unselected rows, footer text).
    pub fg_muted: Color,
    /// Accent color (focus rings, active borders, permission prompts).
    pub accent: Color,
    /// Border color (frame around the menu).
    pub border: Color,
    /// Optional background color for the selected row.
    ///
    /// When `Some`, the active row gets a tinted background; when `None`, the
    /// active row is distinguished only by foreground color + marker.
    pub selected_bg: Option<Color>,
    /// The marker character shown on the selected row.
    ///
    /// Typically `"❯"`, `"›"`, or `"◆"`.
    pub marker: &'static str,
    /// Width of the marker column in cells.
    ///
    /// Most themes use `1`; Grok uses `2` to indent the body past the marker.
    pub marker_width: u16,
}

/// Claude Code palette — light-blue active, gray inactive, rose accent for
/// permission prompts.
///
/// Colors captured from the real CLI's ANSI output
/// (`#afd7ff` = `38;5;153`, `#cd694a` = `38;5;131`).
pub const CLAUDE: Theme = Theme {
    fg: Color::Rgb(0xaf, 0xd7, 0xff),
    fg_dim: Color::Rgb(0xc0, 0xca, 0xf5),
    fg_muted: Color::Rgb(0x94, 0x94, 0x94),
    accent: Color::Rgb(0xcd, 0x69, 0x4a),
    border: Color::Rgb(0xcd, 0x69, 0x4a),
    selected_bg: None,
    marker: "❯",
    marker_width: 1,
};

/// Codex palette — near-white active, gray inactive, cream accent.
pub const CODEX: Theme = Theme {
    fg: Color::Rgb(0xed, 0xed, 0xed),
    fg_dim: Color::Rgb(0xed, 0xed, 0xed),
    fg_muted: Color::Rgb(0x7a, 0x7a, 0x7a),
    accent: Color::Rgb(0xf6, 0xe2, 0xb7),
    border: Color::Rgb(0x7a, 0x7a, 0x7a),
    selected_bg: None,
    marker: "›",
    marker_width: 1,
};

/// Grok palette — white active, gray inactive, silver borders.
pub const GROK: Theme = Theme {
    fg: Color::Rgb(0xe1, 0xe1, 0xe1),
    fg_dim: Color::Rgb(0x8b, 0x8b, 0x90),
    fg_muted: Color::Rgb(0x6c, 0x6c, 0x6c),
    accent: Color::Rgb(0xe1, 0xe1, 0xe1),
    border: Color::Rgb(0x80, 0x80, 0x80),
    selected_bg: None,
    marker: "❯",
    marker_width: 2,
};

impl Default for Theme {
    fn default() -> Self {
        CLAUDE
    }
}

/// Look up a bundled theme by its case-insensitive name.
///
/// Returns `Some(theme)` for `"claude"`, `"codex"`, `"grok"` (and
/// their `SCREAMING_SNAKE_CASE` spellings); `None` otherwise.
///
/// Use this to bridge configuration sources (env vars, config files,
/// CLI flags) to the bundled themes without coupling the spelling to
/// any specific source.
#[must_use]
pub fn lookup(name: &str) -> Option<Theme> {
    match name.to_ascii_lowercase().as_str() {
        "claude" => Some(CLAUDE),
        "codex" => Some(CODEX),
        "grok" => Some(GROK),
        _ => None,
    }
}

impl Theme {
    /// Resolve a theme by name (case-insensitive) — see [`lookup`].
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        lookup(name)
    }

    /// Resolve a theme from the named environment variable, returning
    /// `None` if the variable is unset, empty, or not a recognized
    /// theme name.
    ///
    /// # Example
    ///
    /// ```rust
    /// # // Set the variable in your environment / shell.
    /// # unsafe { std::env::set_var("CORNICHON_THEME", "claude"); }
    /// use cornichon::prelude::*;
    /// let theme = Theme::from_env("CORNICHON_THEME");
    /// assert_eq!(theme, Some(CLAUDE));
    /// # unsafe { std::env::remove_var("CORNICHON_THEME"); }
    /// ```
    #[must_use]
    pub fn from_env(var_name: &str) -> Option<Self> {
        let raw = std::env::var(var_name).ok()?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }
        Self::from_name(trimmed)
    }

    /// Like [`Theme::from_env`] but falls back to `default` when the
    /// env var is unset, empty, or unrecognized.
    ///
    /// # Example
    ///
    /// ```rust
    /// # unsafe { std::env::set_var("CORNICHON_THEME", "claude"); }
    /// use cornichon::prelude::*;
    /// let theme = Theme::from_env_or("CORNICHON_THEME", CODEX);
    /// assert_eq!(theme, CLAUDE);
    /// # unsafe { std::env::remove_var("CORNICHON_THEME"); }
    /// ```
    #[must_use]
    pub fn from_env_or(var_name: &str, default: Self) -> Self {
        Self::from_env(var_name).unwrap_or(default)
    }

    /// All bundled themes in declaration order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[CLAUDE, CODEX, GROK]
    }

    /// The bundled theme names (case-insensitive). Useful for
    /// `--help` strings, settings UIs, and config validation.
    #[must_use]
    pub const fn names() -> &'static [&'static str] {
        &["claude", "codex", "grok"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_claude() {
        assert_eq!(Theme::default(), CLAUDE);
    }

    #[test]
    fn themes_are_distinct() {
        // Sanity check that we didn't accidentally duplicate a palette.
        assert_ne!(CLAUDE, CODEX);
        assert_ne!(CLAUDE, GROK);
        assert_ne!(CODEX, GROK);
    }
}
