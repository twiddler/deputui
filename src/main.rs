use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event},
};
use std::{
    error::Error,
    io::{self},
};

mod app;
mod tui;
use crate::app::App;
use crate::tui::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    match res {
        Ok(true) => app.print_json()?,
        Ok(false) => return Err("Not printing; aborting with exit code 1 …".into()),
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| f.render_widget(&*app, f.area()))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);

            if let Some(action) = app.should_exit {
                return Ok(action);
            }
        }
    }
}
