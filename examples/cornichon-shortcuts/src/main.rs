//! cornichon-shortcuts — demo of [`ShortcutsModal`](cornichon::ShortcutsModal).
//!
//! Run with: `cargo run -p cornichon-shortcuts`
//!
//! Up/Down to move between categories; Enter/Right to expand; Left to
//! collapse; Esc collapses, then on the second press closes the modal.

use anyhow::{Context, Result};
use cornichon::prelude::*;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};

type Term = Terminal<CrosstermBackend<Stdout>>;

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setting up terminal")?;

    let action = {
        let mut modal = ShortcutsModal::with_defaults().context("building shortcuts modal")?;
        run(&mut terminal, &mut modal)
    };

    restore_terminal(&mut terminal).context("restoring terminal")?;

    match action? {
        Some(Outcome::Cancelled) => println!("\nclosed"),
        None => {}
    }
    Ok(())
}

#[derive(Debug)]
enum Outcome {
    Cancelled,
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

fn run(terminal: &mut Term, modal: &mut ShortcutsModal) -> Result<Option<Outcome>> {
    loop {
        terminal
            .draw(|f| f.render_widget(&*modal, f.area()))
            .context("drawing frame")?;

        let Event::Key(key) = event::read().context("reading event")? else {
            continue;
        };

        match modal.update(key) {
            MenuAction::Cancelled => return Ok(Some(Outcome::Cancelled)),
            MenuAction::Selected(_) | MenuAction::None | MenuAction::Activated => {}
        }
    }
}
