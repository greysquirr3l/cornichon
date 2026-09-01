//! cornichon-spinner — demo of every cornichon spinner style.
//!
//! Run with: `cargo run -p cornichon-spinner`
//!
//! Displays 16 spinner styles, each on its own row, all animated at the
//! same wall-clock rate (so you can visually compare their pacing). Press
//! Esc or 'q' to quit.

use anyhow::{Context, Result};
use cornichon::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

type Term = Terminal<CrosstermBackend<Stdout>>;

/// The 16 styles we'll display. Each row uses a different cadence so the
/// pacing visibly differs — slow styles feel slow, fast ones feel fast.
const SHOWCASE: &[(SpinnerStyle, Cadence, &str)] = &[
    (
        SpinnerStyle::Dots,
        Cadence::NORMAL,
        "dots (braille, classic)",
    ),
    (SpinnerStyle::Line, Cadence::NORMAL, "line (ASCII |/-)"),
    (SpinnerStyle::Arc, Cadence::NORMAL, "arc (rotating)"),
    (SpinnerStyle::Arrow, Cadence::FAST, "arrow (8-direction)"),
    (SpinnerStyle::BouncingBar, Cadence::NORMAL, "bouncingBar"),
    (SpinnerStyle::BouncingBall, Cadence::NORMAL, "bouncingBall"),
    (SpinnerStyle::Toggle, Cadence::NORMAL, "toggle (▮▯)"),
    (SpinnerStyle::GrowVertical, Cadence::FAST, "growVertical"),
    (SpinnerStyle::Pulse, Cadence::NORMAL, "pulse"),
    (
        SpinnerStyle::Material,
        Cadence::NORMAL,
        "material (braille)",
    ),
    (SpinnerStyle::Aesthetic, Cadence::NORMAL, "aesthetic (▓▒░)"),
    (SpinnerStyle::Clock, Cadence::SLOW, "clock (12 frame emoji)"),
    (SpinnerStyle::Earth, Cadence::NORMAL, "earth (🌍🌎🌏)"),
    (SpinnerStyle::Moon, Cadence::NORMAL, "moon (8 phase)"),
    (SpinnerStyle::Dqpb, Cadence::FAST, "dqpb (-x|+|)"),
    (SpinnerStyle::Heart, Cadence::SLOW, "heart (5 frame emoji)"),
];

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setting up terminal")?;

    let action = {
        // Build one spinner per row.
        let mut spinners: Vec<Spinner> = SHOWCASE
            .iter()
            .map(|(style, cadence, label)| {
                Spinner::builder()
                    .style(*style)
                    .cadence(*cadence)
                    .label(*label)
                    .build()
            })
            .collect();

        run(&mut terminal, &mut spinners)
    };

    restore_terminal(&mut terminal).context("restoring terminal")?;

    match action? {
        Some(Outcome::Quit) => println!("\nquitting"),
        None => {}
    }
    Ok(())
}

#[derive(Debug)]
enum Outcome {
    Quit,
}

fn setup_terminal() -> Result<Term> {
    enable_raw_mode().context("enabling raw mode")?;
    execute!(io::stdout(), EnterAlternateScreen).context("entering alternate screen")?;
    let backend = CrosstermBackend::new(io::stdout());
    Terminal::new(backend).context("creating terminal")
}

fn restore_terminal(terminal: &mut Term) -> Result<()> {
    disable_raw_mode().context("disabling raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).context("leaving alternate screen")?;
    terminal.show_cursor().context("showing cursor")?;
    Ok(())
}

/// Run the demo: drive each spinner from absolute wall-clock elapsed time,
/// redraw at ~60 fps, accept Esc/q to quit.
///
/// Per-iteration `advance(Duration)` won't work here because each frame's
/// elapsed (≈16 ms) is shorter than every `Cadence` value — `advance`
/// would return `0` and the spinners would freeze. Instead we set each
/// spinner's tick from total uptime every frame, so animation is
/// independent of render frequency.
fn run(terminal: &mut Term, spinners: &mut [Spinner]) -> Result<Option<Outcome>> {
    let start = Instant::now();
    let frame = Duration::from_millis(16); // ~60 fps redraw
    loop {
        // Drive each spinner from total uptime — independent of the
        // renderer's frame rate.
        let total = start.elapsed();
        for s in spinners.iter_mut() {
            s.tick_at(total);
        }

        terminal
            .draw(|f| {
                let area = f.area();
                let title = "cornichon spinner styles — press q or Esc to quit";
                f.render_widget(
                    ratatui::widgets::Paragraph::new(title)
                        .style(ratatui::style::Style::default().fg(Theme::default().fg)),
                    ratatui::layout::Rect {
                        x: area.x,
                        y: area.y,
                        width: area.width,
                        height: 1,
                    },
                );
                for (i, s) in spinners.iter().enumerate() {
                    let y = area
                        .y
                        .saturating_add(2)
                        .saturating_add(u16::try_from(i).unwrap_or(u16::MAX));
                    if y >= area.y + area.height {
                        break;
                    }
                    f.render_widget(
                        s,
                        ratatui::layout::Rect {
                            x: area.x + 2,
                            y,
                            width: area.width.saturating_sub(2),
                            height: 1,
                        },
                    );
                }
            })
            .context("drawing frame")?;

        // Non-blocking poll, then throttle to ~60 fps. Polling with
        // ZERO duration returns immediately if no events are pending.
        if event::poll(Duration::ZERO).context("polling events")? {
            let Event::Key(key) = event::read().context("reading event")? else {
                std::thread::sleep(frame);
                continue;
            };
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => return Ok(Some(Outcome::Quit)),
                _ => {}
            }
        }
        std::thread::sleep(frame);
    }
}
