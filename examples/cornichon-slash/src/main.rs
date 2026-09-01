//! cornichon-slash — demo of [`SlashMenu`](cornichon::SlashMenu).
//!
//! Run with: `cargo run -p cornichon-slash`
//!
//! Type characters to filter; Up/Down to navigate; Enter to confirm; Esc
//! to cancel.

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
        let mut menu = SlashMenu::with_defaults().context("building slash menu")?;
        run(&mut terminal, &mut menu)
    };

    restore_terminal(&mut terminal).context("restoring terminal")?;

    match action? {
        Some(Outcome::Selected(name)) => println!("\nselected: {name}"),
        Some(Outcome::Cancelled) => println!("\ncancelled"),
        None => {}
    }
    Ok(())
}

#[derive(Debug)]
enum Outcome {
    Selected(String),
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

fn run(terminal: &mut Term, menu: &mut SlashMenu) -> Result<Option<Outcome>> {
    loop {
        terminal
            .draw(|f| f.render_widget(&*menu, f.area()))
            .context("drawing frame")?;

        let Event::Key(key) = event::read().context("reading event")? else {
            continue;
        };

        match menu.update(key) {
            MenuAction::Selected(item) => return Ok(Some(Outcome::Selected(item.name))),
            MenuAction::Cancelled => return Ok(Some(Outcome::Cancelled)),
            MenuAction::None | MenuAction::Activated => {}
        }
    }
}
