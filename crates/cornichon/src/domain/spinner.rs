//! Spinner primitives — style catalog, cadence, tick counter.
//!
//! [`SpinnerStyle`] enumerates every common CLI spinner animation. Names
//! follow the [cli-spinners](https://github.com/sindresorhus/cli-spinners)
//! convention so muscle memory from JS tooling carries over.
//!
//! Coverage (61 variants across 9 families):
//!
//! - **Dots** (12) — `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` family
//! - **Line** (2) — classic `|/-\`
//! - **Geometry** (8) — arc, arrows, quadrants, triangle, square corners, circle halves
//! - **Toggle / Block** (6) — `▮▯`-style variants
//! - **Bouncing / Bars** (5) — bouncing bar/ball, hamburger, grow-vertical/horizontal
//! - **Pulse / Progress** (5) — pulse, points, binary, pipe, layer
//! - **Noise / Decoration** (6) — aesthetic, material(2), noise, squish, star
//! - **Emoji** (13) — clock, earth, moon, weather, christmas, monkey, runner, pong,
//!   shark, dqpb, mind-blown, speaker, heart
//! - **Special** (4) — bounce, simple dots (3 variants)
//!
//! Use [`crate::application::spinner::Spinner`] for the state machine; this
//! module only describes *what* to animate, not *when*.

use core::time::Duration;

/// One frame of animation cadence.
///
/// `Cadence(80)` matches the common rustc/cargo spinner rate (12.5 fps).
/// `Cadence::FAST` is for "active / busy" loops, `Cadence::SLOW` for
/// "thoughtful / idle" loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cadence(pub u16);

impl Cadence {
    /// 50 ms — busy / active.
    pub const FAST: Self = Self(50);
    /// 80 ms — typical default (matches rustc / cargo).
    pub const NORMAL: Self = Self(80);
    /// 130 ms — relaxed / thoughtful.
    pub const SLOW: Self = Self(130);

    /// Frame interval in milliseconds.
    #[must_use]
    pub const fn millis(self) -> u16 {
        self.0
    }

    /// Construct from a [`Duration`]. Sub-millisecond durations round up to 1.
    #[must_use]
    pub fn from_duration(d: Duration) -> Self {
        let ms = u16::try_from(d.as_millis()).unwrap_or(u16::MAX);
        Self(if ms == 0 { 1 } else { ms })
    }
}

impl Default for Cadence {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// Monotonic tick counter, incremented every `Cadence::millis()`.
///
/// Wraps modulo the active spinner's `frames().len()` to select a frame.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tick(pub u32);

impl Tick {
    /// Zero — the initial tick, before any animation has advanced.
    pub const ZERO: Self = Self(0);

    /// Advance to the next tick.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
    }

    /// Increment in place.
    pub const fn advance(&mut self) {
        *self = self.next();
    }
}

/// The catalog of shipped spinner animations.
///
/// Each variant is identified by a stable snake-case name (see
/// [`SpinnerStyle::name`]) and resolves to a `&'static [&'static str]` of
/// frame glyphs (see [`SpinnerStyle::frames`]).
///
/// Use [`crate::application::spinner::Spinner`] for the state machine; this
/// enum only describes *what* to animate, not *when*.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)] // CI / DQPB are conventional names
pub enum SpinnerStyle {
    // ----- Dots family (braille-based) ------------------------------------
    /// Classic 10-frame braille spinner.
    #[default]
    Dots,
    /// 11-frame clockwise braille.
    Dots2,
    /// 11-frame counter-clockwise braille.
    Dots3,
    /// 7-frame grow-once braille.
    Dots4,
    /// 7-frame grow-then-stay braille.
    Dots5,
    /// 10-frame braille glow.
    Dots6,
    /// 6-frame middle-out braille.
    Dots7,
    /// 6-frame pulse-out braille.
    Dots8,
    /// 8-frame staircase.
    Dots9,
    /// 8-frame top-down triangle braille.
    Dots10,
    /// 8-frame staircase reverse braille.
    Dots11,
    /// 8-frame staircase grow braille.
    Dots12,

    // ----- Line family -----------------------------------------------------
    /// Classic ASCII line.
    Line,
    /// ASCII line, alternating direction.
    Line2,

    // ----- Geometry --------------------------------------------------------
    /// Rotating arc.
    Arc,
    /// 8-direction arrow.
    Arrow,
    /// 4-direction arrow.
    Arrow2,
    /// 2-frame arrow.
    Arrow3,
    /// Rotating quadrants.
    Quadrants,
    /// Rotating triangle.
    Triangle,
    /// Rotating square corners.
    SquareCorners,
    /// Rotating circle halves.
    CircleHalves,

    // ----- Toggle / Block --------------------------------------------------
    /// 2-frame solid / hollow.
    Toggle,
    /// 2-frame shaded / dim.
    Toggle2,
    /// 2-frame empty / filled.
    Toggle3,
    /// 2-frame dotted / solid.
    Toggle4,
    /// 2-frame striped / dim.
    Toggle5,
    /// 2-frame unemphasized / emphasized.
    Toggle6,

    // ----- Bouncing / Bars -------------------------------------------------
    /// Bouncing bar in brackets (14-frame).
    BouncingBar,
    /// Bouncing ball in parens (10-frame).
    BouncingBall,
    /// Hamburger menu icon morph.
    Hamburger,
    /// Grow-vertical bar (8-frame).
    GrowVertical,
    /// Grow-horizontal bar (8-frame).
    GrowHorizontal,

    // ----- Pulse / Progress ------------------------------------------------
    /// 5-frame pulse.
    Pulse,
    /// 3-frame point grow.
    Points,
    /// 8-frame binary.
    Binary,
    /// Rotating pipe.
    Pipe,
    /// Growing equal layer.
    Layer,

    // ----- Noise / Decoration ----------------------------------------------
    /// ASCII noise fade.
    Aesthetic,
    /// 9-frame braille material.
    Material,
    /// 8-frame braille material reverse.
    Material2,
    /// Braille noise.
    Noise,
    /// Scissor close / open.
    Squish,
    /// 5-frame star.
    Star,

    // ----- Emoji -----------------------------------------------------------
    /// 12-frame clock face.
    Clock,
    /// Rotating earth.
    Earth,
    /// 8-frame moon phases.
    Moon,
    /// 4-frame weather.
    Weather,
    /// 3-frame christmas.
    Christmas,
    /// 3-frame see-no / hear-no / speak-no.
    Monkey,
    /// 4-frame runner morph.
    Runner,
    /// 2-frame pong bat.
    Pong,
    /// 3-frame shark.
    Shark,
    /// 4-frame dot-quad-bit.
    Dqpb,
    /// Single-frame mind-blown.
    MindBlown,
    /// 4-frame speaker volume.
    Speaker,
    /// 5-frame hearts.
    Heart,

    // ----- Special ---------------------------------------------------------
    /// Braille bounce.
    Bounce,
    /// 4-frame simple dots.
    SimpleDots,
    /// 6-frame scrolling single dots.
    SimpleDotsScrolling,
    /// 6-frame typing dots.
    SimpleDotsTyping,
}

impl SpinnerStyle {
    /// Stable snake-case identifier (matches `cli-spinners`).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dots => "dots",
            Self::Dots2 => "dots2",
            Self::Dots3 => "dots3",
            Self::Dots4 => "dots4",
            Self::Dots5 => "dots5",
            Self::Dots6 => "dots6",
            Self::Dots7 => "dots7",
            Self::Dots8 => "dots8",
            Self::Dots9 => "dots9",
            Self::Dots10 => "dots10",
            Self::Dots11 => "dots11",
            Self::Dots12 => "dots12",
            Self::Line => "line",
            Self::Line2 => "line2",
            Self::Arc => "arc",
            Self::Arrow => "arrow",
            Self::Arrow2 => "arrow2",
            Self::Arrow3 => "arrow3",
            Self::Quadrants => "quadrants",
            Self::Triangle => "triangle",
            Self::SquareCorners => "squareCorners",
            Self::CircleHalves => "circleHalves",
            Self::Toggle => "toggle",
            Self::Toggle2 => "toggle2",
            Self::Toggle3 => "toggle3",
            Self::Toggle4 => "toggle4",
            Self::Toggle5 => "toggle5",
            Self::Toggle6 => "toggle6",
            Self::BouncingBar => "bouncingBar",
            Self::BouncingBall => "bouncingBall",
            Self::Hamburger => "hamburger",
            Self::GrowVertical => "growVertical",
            Self::GrowHorizontal => "growHorizontal",
            Self::Pulse => "pulse",
            Self::Points => "points",
            Self::Binary => "binary",
            Self::Pipe => "pipe",
            Self::Layer => "layer",
            Self::Aesthetic => "aesthetic",
            Self::Material => "material",
            Self::Material2 => "material2",
            Self::Noise => "noise",
            Self::Squish => "squish",
            Self::Star => "star",
            Self::Clock => "clock",
            Self::Earth => "earth",
            Self::Moon => "moon",
            Self::Weather => "weather",
            Self::Christmas => "christmas",
            Self::Monkey => "monkey",
            Self::Runner => "runner",
            Self::Pong => "pong",
            Self::Shark => "shark",
            Self::Dqpb => "dqpb",
            Self::MindBlown => "mindBlown",
            Self::Speaker => "speaker",
            Self::Heart => "heart",
            Self::Bounce => "bounce",
            Self::SimpleDots => "simpleDots",
            Self::SimpleDotsScrolling => "simpleDotsScrolling",
            Self::SimpleDotsTyping => "simpleDotsTyping",
        }
    }

    /// The animation frames for this style. `&'static [&'static str]` lives
    /// in a `const` table — no allocation, no clones.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "exhaustive one-arm-per-variant dispatch; splitting across helpers would scatter SpinnerStyle to frame-set ownership without reducing line count"
    )]
    pub const fn frames(self) -> &'static [&'static str] {
        const DOTS: &[&str] = &[
            "\u{280B}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283C}", "\u{2834}", "\u{2826}",
            "\u{2827}", "\u{2807}", "\u{280F}",
        ];
        const DOTS2: &[&str] = &[
            "\u{2801}", "\u{2809}", "\u{2819}", "\u{281A}", "\u{281E}", "\u{2816}", "\u{2826}",
            "\u{2834}", "\u{2832}", "\u{2833}", "\u{2813}",
        ];
        const DOTS3: &[&str] = &[
            "\u{2804}", "\u{2806}", "\u{2807}", "\u{280B}", "\u{2819}", "\u{2838}", "\u{2830}",
            "\u{2820}", "\u{2830}", "\u{2838}", "\u{2819}",
        ];
        const DOTS4: &[&str] = &[
            "\u{2801}", "\u{2803}", "\u{2807}", "\u{2817}", "\u{2837}", "\u{283F}", "\u{28FF}",
        ];
        const DOTS5: &[&str] = &[
            "\u{2801}", "\u{2803}", "\u{2807}", "\u{280F}", "\u{281F}", "\u{283F}", "\u{28FF}",
        ];
        const DOTS6: &[&str] = &[
            "\u{2880}", "\u{2881}", "\u{2882}", "\u{2883}", "\u{2884}", "\u{2885}", "\u{2886}",
            "\u{2887}", "\u{2888}", "\u{2889}",
        ];
        const DOTS7: &[&str] = &[
            "\u{2880}", "\u{2884}", "\u{2886}", "\u{2886}", "\u{2886}", "\u{2880}",
        ];
        const DOTS8: &[&str] = &[
            "\u{2802}", "\u{2818}", "\u{2808}", "\u{2808}", "\u{2818}", "\u{2802}",
        ];
        const DOTS9: &[&str] = &[
            "\u{2801}", "\u{2803}", "\u{2804}", "\u{2802}", "\u{2801}", "\u{2803}", "\u{2804}",
            "\u{2802}",
        ];
        const DOTS10: &[&str] = &[
            "\u{2880}", "\u{2880}", "\u{28E4}", "\u{28C4}", "\u{2820}", "\u{28C4}", "\u{28E4}",
            "\u{2880}",
        ];
        const DOTS11: &[&str] = &[
            "\u{2801}", "\u{2802}", "\u{2804}", "\u{2840}", "\u{2880}", "\u{2804}", "\u{2802}",
            "\u{2801}",
        ];
        const DOTS12: &[&str] = &[
            "\u{2880}", "\u{2880}", "\u{2804}", "\u{2802}", "\u{2801}", "\u{2802}", "\u{2804}",
            "\u{2804}",
        ];
        const LINE: &[&str] = &["|", "/", "-", "\\"];
        const LINE2: &[&str] = &["-", "\\", "|", "/", "-"];
        const ARC: &[&str] = &[
            "\u{25DC}", "\u{25E0}", "\u{25DD}", "\u{25DE}", "\u{25E1}", "\u{25DF}",
        ];
        const ARROW: &[&str] = &[
            "\u{2190}", "\u{2196}", "\u{2191}", "\u{2197}", "\u{2192}", "\u{2198}", "\u{2193}",
            "\u{2199}",
        ];
        const ARROW2: &[&str] = &["\u{2190}", "\u{2191}", "\u{2192}", "\u{2193}"];
        const ARROW3: &[&str] = &["\u{25B9}", "\u{25B8}"];
        const QUADRANTS: &[&str] = &["\u{2596}", "\u{2598}", "\u{259D}", "\u{2597}"];
        const TRIANGLE: &[&str] = &["\u{25E2}", "\u{25E3}", "\u{25E4}", "\u{25E5}"];
        const SQUARE_CORNERS: &[&str] = &["\u{25F0}", "\u{25F3}", "\u{25F2}", "\u{25F1}"];
        const CIRCLE_HALVES: &[&str] = &["\u{25D0}", "\u{25D1}", "\u{25D2}", "\u{25D3}"];
        const TOGGLE: &[&str] = &["\u{25AE}", "\u{25AF}"];
        const TOGGLE2: &[&str] = &["\u{25AB}", "\u{25AA}"];
        const TOGGLE3: &[&str] = &["\u{25A1}", "\u{25A0}"];
        const TOGGLE4: &[&str] = &["\u{25CC}", "\u{25CD}"];
        const TOGGLE5: &[&str] = &["\u{25B0}", "\u{25B1}"];
        const TOGGLE6: &[&str] = &["\u{25FD}", "\u{25FE}"];
        const BOUNCING_BAR: &[&str] = &[
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
            "[   =]", "[  ==]", "[ ===]", "[=== ]", "[==  ]", "[=   ]",
        ];
        const BOUNCING_BALL: &[&str] = &[
            "( \u{25CF}    )",
            "(  \u{25CF}   )",
            "(   \u{25CF}  )",
            "(    \u{25CF} )",
            "(     \u{25CF})",
            "(    \u{25CF} )",
            "(   \u{25CF}  )",
            "(  \u{25CF}   )",
            "( \u{25CF}    )",
            "(\u{25CF}     )",
        ];
        const HAMBURGER: &[&str] = &["\u{2631}", "\u{2632}", "\u{2634}"];
        const GROW_VERTICAL: &[&str] = &[
            "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}",
            "\u{2588}",
        ];
        const GROW_HORIZONTAL: &[&str] = &[
            "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}",
            "\u{2588}",
        ];
        const PULSE: &[&str] = &[
            "\u{2588}\u{2592}",
            "\u{2592}\u{2592}",
            "\u{2592}\u{2591}",
            "\u{2591}\u{2591}",
            "\u{2591}\u{2588}",
        ];
        const POINTS: &[&str] = &["\u{2219}", "\u{2022}", "\u{25CF}"];
        const BINARY: &[&str] = &["0", "1", "0", "1", " ", "1", "0", "1"];
        const PIPE: &[&str] = &["\u{2524}", "\u{2518}", "\u{2510}", "\u{250C}", "\u{250C}"];
        const LAYER: &[&str] = &["-", "=", "\u{2261}"];
        const AESTHETIC: &[&str] = &["\u{2593}", "\u{2592}", "\u{2591}"];
        const MATERIAL: &[&str] = &[
            "\u{28BE}", "\u{28BD}", "\u{28BB}", "\u{28BF}", "\u{283F}", "\u{28FF}", "\u{283F}",
            "\u{281F}", "\u{280F}",
        ];
        const MATERIAL2: &[&str] = &[
            "\u{28FF}", "\u{283F}", "\u{28BF}", "\u{28BB}", "\u{28BD}", "\u{28BE}",
        ];
        const NOISE: &[&str] = &["\u{2801}", "\u{2802}", "\u{2804}", "\u{2802}"];
        const SQUISH: &[&str] = &["\u{2282}", "\u{2283}"];
        const STAR: &[&str] = &["\u{2736}", "\u{2738}", "\u{273A}", "\u{2739}", "\u{2737}"];
        const CLOCK: &[&str] = &[
            "\u{1F550}",
            "\u{1F551}",
            "\u{1F552}",
            "\u{1F553}",
            "\u{1F554}",
            "\u{1F555}",
            "\u{1F556}",
            "\u{1F557}",
            "\u{1F558}",
            "\u{1F559}",
            "\u{1F55A}",
            "\u{1F55B}",
        ];
        const EARTH: &[&str] = &["\u{1F30D}", "\u{1F30E}", "\u{1F30F}"];
        const MOON: &[&str] = &[
            "\u{1F311}",
            "\u{1F312}",
            "\u{1F313}",
            "\u{1F314}",
            "\u{1F315}",
            "\u{1F316}",
            "\u{1F317}",
            "\u{1F318}",
        ];
        const WEATHER: &[&str] = &[
            "\u{2600}\u{FE0F}",
            "\u{26C5}",
            "\u{2601}\u{FE0F}",
            "\u{1F327}\u{FE0F}",
        ];
        const CHRISTMAS: &[&str] = &["\u{1F384}", "\u{1F381}", "\u{1F385}"];
        const MONKEY: &[&str] = &["\u{1F648}", "\u{1F649}", "\u{1F64A}"];
        const RUNNER: &[&str] = &["\u{1F6B6}", "\u{1F3C3}", "\u{1F483}", "\u{1F57A}"];
        const PONG: &[&str] = &["\u{2590}", "\u{258C}"];
        const SHARK: &[&str] = &["\u{2590}\u{258C}", "\u{258C}\u{2590}", "\u{2590}\u{258C}"];
        const DQPB: &[&str] = &["-", "x", "+", "x"];
        const MIND_BLOWN: &[&str] = &["\u{1F92F}"];
        const SPEAKER: &[&str] = &["\u{1F50A}", "\u{1F509}", "\u{1F508}", "\u{1F507}"];
        const HEART: &[&str] = &[
            "\u{2764}",
            "\u{1F497}",
            "\u{1F496}",
            "\u{1F49D}",
            "\u{1F498}",
        ];
        const BOUNCE: &[&str] = &["\u{2801}", "\u{2802}", "\u{2804}", "\u{2802}"];
        const SIMPLE_DOTS: &[&str] = &[".  ", ".. ", "...", "   "];
        const SIMPLE_DOTS_SCROLLING: &[&str] = &[".  ", ".. ", "...", " ..", "  .", "   "];
        const SIMPLE_DOTS_TYPING: &[&str] = &[".  ", ".. ", "...", " ..", "  .", "   "];

        match self {
            Self::Dots => DOTS,
            Self::Dots2 => DOTS2,
            Self::Dots3 => DOTS3,
            Self::Dots4 => DOTS4,
            Self::Dots5 => DOTS5,
            Self::Dots6 => DOTS6,
            Self::Dots7 => DOTS7,
            Self::Dots8 => DOTS8,
            Self::Dots9 => DOTS9,
            Self::Dots10 => DOTS10,
            Self::Dots11 => DOTS11,
            Self::Dots12 => DOTS12,
            Self::Line => LINE,
            Self::Line2 => LINE2,
            Self::Arc => ARC,
            Self::Arrow => ARROW,
            Self::Arrow2 => ARROW2,
            Self::Arrow3 => ARROW3,
            Self::Quadrants => QUADRANTS,
            Self::Triangle => TRIANGLE,
            Self::SquareCorners => SQUARE_CORNERS,
            Self::CircleHalves => CIRCLE_HALVES,
            Self::Toggle => TOGGLE,
            Self::Toggle2 => TOGGLE2,
            Self::Toggle3 => TOGGLE3,
            Self::Toggle4 => TOGGLE4,
            Self::Toggle5 => TOGGLE5,
            Self::Toggle6 => TOGGLE6,
            Self::BouncingBar => BOUNCING_BAR,
            Self::BouncingBall => BOUNCING_BALL,
            Self::Hamburger => HAMBURGER,
            Self::GrowVertical => GROW_VERTICAL,
            Self::GrowHorizontal => GROW_HORIZONTAL,
            Self::Pulse => PULSE,
            Self::Points => POINTS,
            Self::Binary => BINARY,
            Self::Pipe => PIPE,
            Self::Layer => LAYER,
            Self::Aesthetic => AESTHETIC,
            Self::Material => MATERIAL,
            Self::Material2 => MATERIAL2,
            Self::Noise => NOISE,
            Self::Squish => SQUISH,
            Self::Star => STAR,
            Self::Clock => CLOCK,
            Self::Earth => EARTH,
            Self::Moon => MOON,
            Self::Weather => WEATHER,
            Self::Christmas => CHRISTMAS,
            Self::Monkey => MONKEY,
            Self::Runner => RUNNER,
            Self::Pong => PONG,
            Self::Shark => SHARK,
            Self::Dqpb => DQPB,
            Self::MindBlown => MIND_BLOWN,
            Self::Speaker => SPEAKER,
            Self::Heart => HEART,
            Self::Bounce => BOUNCE,
            Self::SimpleDots => SIMPLE_DOTS,
            Self::SimpleDotsScrolling => SIMPLE_DOTS_SCROLLING,
            Self::SimpleDotsTyping => SIMPLE_DOTS_TYPING,
        }
    }

    /// Number of frames in the animation.
    #[must_use]
    pub const fn frame_count(self) -> usize {
        self.frames().len()
    }

    /// The frame at index `tick.0` (modulo the frame count).
    ///
    /// Not `const fn` because the safe slice-access path uses
    /// [`Option::unwrap_or`], which isn't yet stable as const on stable
    /// Rust. The performance cost is identical — `O(1)` pointer arithmetic
    /// — and `frame_at` is called once per render, not in a hot loop.
    #[must_use]
    pub fn frame_at(self, tick: Tick) -> &'static str {
        let frames = self.frames();
        let len = frames.len();
        if len == 0 {
            return "";
        }
        let idx = (tick.0 as usize) % len;
        // `idx < len` is enforced by the `% len` above; the `unwrap_or("")`
        // is purely defensive and unreachable.
        frames.get(idx).copied().unwrap_or("")
    }

    /// All style names (in enum declaration order).
    #[must_use]
    pub const fn all_names() -> &'static [&'static str] {
        &[
            "dots",
            "dots2",
            "dots3",
            "dots4",
            "dots5",
            "dots6",
            "dots7",
            "dots8",
            "dots9",
            "dots10",
            "dots11",
            "dots12",
            "line",
            "line2",
            "arc",
            "arrow",
            "arrow2",
            "arrow3",
            "quadrants",
            "triangle",
            "squareCorners",
            "circleHalves",
            "toggle",
            "toggle2",
            "toggle3",
            "toggle4",
            "toggle5",
            "toggle6",
            "bouncingBar",
            "bouncingBall",
            "hamburger",
            "growVertical",
            "growHorizontal",
            "pulse",
            "points",
            "binary",
            "pipe",
            "layer",
            "aesthetic",
            "material",
            "material2",
            "noise",
            "squish",
            "star",
            "clock",
            "earth",
            "moon",
            "weather",
            "christmas",
            "monkey",
            "runner",
            "pong",
            "shark",
            "dqpb",
            "mindBlown",
            "speaker",
            "heart",
            "bounce",
            "simpleDots",
            "simpleDotsScrolling",
            "simpleDotsTyping",
        ]
    }

    /// Look up a style by its stable snake-case name. Style lookup is a
    /// one-shot startup concern, not a hot path — `const fn` isn't worth
    /// waiting on `PartialEq` for.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let names = Self::all_names();
        let mut i = 0;
        while i < names.len() {
            if let Some(n) = names.get(i)
                && *n == name
            {
                return Self::from_usize(i);
            }
            i += 1;
        }
        None
    }

    /// Internal: convert a usize index in `[0, all_names().len())` to a style.
    const fn from_usize(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Dots),
            1 => Some(Self::Dots2),
            2 => Some(Self::Dots3),
            3 => Some(Self::Dots4),
            4 => Some(Self::Dots5),
            5 => Some(Self::Dots6),
            6 => Some(Self::Dots7),
            7 => Some(Self::Dots8),
            8 => Some(Self::Dots9),
            9 => Some(Self::Dots10),
            10 => Some(Self::Dots11),
            11 => Some(Self::Dots12),
            12 => Some(Self::Line),
            13 => Some(Self::Line2),
            14 => Some(Self::Arc),
            15 => Some(Self::Arrow),
            16 => Some(Self::Arrow2),
            17 => Some(Self::Arrow3),
            18 => Some(Self::Quadrants),
            19 => Some(Self::Triangle),
            20 => Some(Self::SquareCorners),
            21 => Some(Self::CircleHalves),
            22 => Some(Self::Toggle),
            23 => Some(Self::Toggle2),
            24 => Some(Self::Toggle3),
            25 => Some(Self::Toggle4),
            26 => Some(Self::Toggle5),
            27 => Some(Self::Toggle6),
            28 => Some(Self::BouncingBar),
            29 => Some(Self::BouncingBall),
            30 => Some(Self::Hamburger),
            31 => Some(Self::GrowVertical),
            32 => Some(Self::GrowHorizontal),
            33 => Some(Self::Pulse),
            34 => Some(Self::Points),
            35 => Some(Self::Binary),
            36 => Some(Self::Pipe),
            37 => Some(Self::Layer),
            38 => Some(Self::Aesthetic),
            39 => Some(Self::Material),
            40 => Some(Self::Material2),
            41 => Some(Self::Noise),
            42 => Some(Self::Squish),
            43 => Some(Self::Star),
            44 => Some(Self::Clock),
            45 => Some(Self::Earth),
            46 => Some(Self::Moon),
            47 => Some(Self::Weather),
            48 => Some(Self::Christmas),
            49 => Some(Self::Monkey),
            50 => Some(Self::Runner),
            51 => Some(Self::Pong),
            52 => Some(Self::Shark),
            53 => Some(Self::Dqpb),
            54 => Some(Self::MindBlown),
            55 => Some(Self::Speaker),
            56 => Some(Self::Heart),
            57 => Some(Self::Bounce),
            58 => Some(Self::SimpleDots),
            59 => Some(Self::SimpleDotsScrolling),
            60 => Some(Self::SimpleDotsTyping),
            _ => None,
        }
    }
}

/// Errors that can occur when parsing a [`SpinnerStyle`] from a string.
///
/// Used by [`SpinnerStyle::from_name`]'s [`TryFrom`](core::convert::TryFrom)
/// implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownSpinnerError {
    /// The provided name didn't match any shipped spinner style.
    Unknown,
}

impl core::fmt::Display for UnknownSpinnerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unknown => f.write_str("unknown spinner style name"),
        }
    }
}

impl std::error::Error for UnknownSpinnerError {}

impl TryFrom<&str> for SpinnerStyle {
    type Error = UnknownSpinnerError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Self::from_name(name).ok_or(UnknownSpinnerError::Unknown)
    }
}

impl TryFrom<String> for SpinnerStyle {
    type Error = UnknownSpinnerError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::try_from(name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_dots() {
        assert_eq!(SpinnerStyle::default(), SpinnerStyle::Dots);
    }

    #[test]
    fn every_variant_has_at_least_one_frame() {
        let all: &[SpinnerStyle] = &[
            SpinnerStyle::Dots,
            SpinnerStyle::Dots2,
            SpinnerStyle::Dots3,
            SpinnerStyle::Dots4,
            SpinnerStyle::Dots5,
            SpinnerStyle::Dots6,
            SpinnerStyle::Dots7,
            SpinnerStyle::Dots8,
            SpinnerStyle::Dots9,
            SpinnerStyle::Dots10,
            SpinnerStyle::Dots11,
            SpinnerStyle::Dots12,
            SpinnerStyle::Line,
            SpinnerStyle::Line2,
            SpinnerStyle::Arc,
            SpinnerStyle::Arrow,
            SpinnerStyle::Arrow2,
            SpinnerStyle::Arrow3,
            SpinnerStyle::Quadrants,
            SpinnerStyle::Triangle,
            SpinnerStyle::SquareCorners,
            SpinnerStyle::CircleHalves,
            SpinnerStyle::Toggle,
            SpinnerStyle::Toggle2,
            SpinnerStyle::Toggle3,
            SpinnerStyle::Toggle4,
            SpinnerStyle::Toggle5,
            SpinnerStyle::Toggle6,
            SpinnerStyle::BouncingBar,
            SpinnerStyle::BouncingBall,
            SpinnerStyle::Hamburger,
            SpinnerStyle::GrowVertical,
            SpinnerStyle::GrowHorizontal,
            SpinnerStyle::Pulse,
            SpinnerStyle::Points,
            SpinnerStyle::Binary,
            SpinnerStyle::Pipe,
            SpinnerStyle::Layer,
            SpinnerStyle::Aesthetic,
            SpinnerStyle::Material,
            SpinnerStyle::Material2,
            SpinnerStyle::Noise,
            SpinnerStyle::Squish,
            SpinnerStyle::Star,
            SpinnerStyle::Clock,
            SpinnerStyle::Earth,
            SpinnerStyle::Moon,
            SpinnerStyle::Weather,
            SpinnerStyle::Christmas,
            SpinnerStyle::Monkey,
            SpinnerStyle::Runner,
            SpinnerStyle::Pong,
            SpinnerStyle::Shark,
            SpinnerStyle::Dqpb,
            SpinnerStyle::MindBlown,
            SpinnerStyle::Speaker,
            SpinnerStyle::Heart,
            SpinnerStyle::Bounce,
            SpinnerStyle::SimpleDots,
            SpinnerStyle::SimpleDotsScrolling,
            SpinnerStyle::SimpleDotsTyping,
        ];
        for s in all {
            assert!(!s.frames().is_empty(), "{} has no frames", s.name());
        }
    }

    #[test]
    fn frame_at_wraps_around() {
        let s = SpinnerStyle::Dots; // 10 frames

        // Tick(0) and Tick(10) both map to frame 0.
        assert_eq!(s.frame_at(Tick(0)), s.frame_at(Tick(10)));

        // Tick(n) and Tick(n + 10) always wrap to the same frame.
        for n in 0..10 {
            assert_eq!(s.frame_at(Tick(n)), s.frame_at(Tick(n + 10)));
        }

        // Different ticks can give different frames.
        assert_ne!(s.frame_at(Tick(0)), s.frame_at(Tick(1)));
    }

    #[test]
    fn from_name_round_trips() -> Result<(), UnknownSpinnerError> {
        for n in SpinnerStyle::all_names() {
            let s = SpinnerStyle::try_from(*n)?;
            assert_eq!(s.name(), *n);
        }
        Ok(())
    }

    #[test]
    fn from_name_rejects_unknown() {
        let err = SpinnerStyle::try_from("not-a-real-spinner");
        assert_eq!(err, Err(UnknownSpinnerError::Unknown));
    }

    #[test]
    fn cadence_default_is_normal() {
        assert_eq!(Cadence::default(), Cadence::NORMAL);
        assert_eq!(Cadence::NORMAL.millis(), 80);
        assert_eq!(Cadence::FAST.millis(), 50);
        assert_eq!(Cadence::SLOW.millis(), 130);
    }

    #[test]
    fn cadence_from_duration_clamps_to_one_ms_minimum() {
        assert_eq!(Cadence::from_duration(Duration::ZERO).millis(), 1);
        assert_eq!(
            Cadence::from_duration(Duration::from_millis(200)).millis(),
            200,
        );
    }

    #[test]
    fn tick_advances() {
        let mut t = Tick::ZERO;
        t.advance();
        assert_eq!(t, Tick(1));
        t.advance();
        assert_eq!(t, Tick(2));
    }
}
