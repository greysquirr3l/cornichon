//! cornichon-picker — demo of [`ProjectPicker`](cornichon::ProjectPicker).
//!
//! Run with: `cargo run -p cornichon-picker`
//!
//! Press 1/2/3 to pick a project; press z (or Down past the last project)
//! to pick "custom"; Esc to cancel.

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
        let mut picker = ProjectPicker::with_defaults().context("building picker")?;
        run(&mut terminal, &mut picker)
    };

    restore_terminal(&mut terminal).context("restoring terminal")?;

    match action? {
        Some(Outcome::Selected(name)) => println!("\nproject: {name}"),
        Some(Outcome::Custom) => println!("\ncustom input requested"),
        Some(Outcome::Cancelled) => println!("\ncancelled"),
        None => {}
    }
    Ok(())
}

#[derive(Debug)]
enum Outcome {
    Selected(String),
    Custom,
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

fn run(terminal: &mut Term, picker: &mut ProjectPicker) -> Result<Option<Outcome>> {
    loop {
        terminal
            .draw(|f| f.render_widget(&*picker, f.area()))
            .context("drawing frame")?;

        let Event::Key(key) = event::read().context("reading event")? else {
            continue;
        };

        match picker.update(key) {
            MenuAction::Selected(item) => return Ok(Some(Outcome::Selected(item.name))),
            MenuAction::Activated => return Ok(Some(Outcome::Custom)),
            MenuAction::Cancelled => return Ok(Some(Outcome::Cancelled)),
            MenuAction::None => {}
        }
    }
}
