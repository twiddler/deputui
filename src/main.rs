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

    let values = vec!["foo", "bar", "baz"];
    let mut app = App::new(&values);
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    match res {
        Ok(true) => {
            let output = app.get_selection();
            println!("{output}");
        }
        Ok(false) => return Err("Not printing; aborting with exit code 1 …".into()),
        Err(err) => return Err(err.into()),
    }

    Ok(())
}

fn run_app<B: Backend<Error = io::Error>>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| frame.render_widget(&*app, frame.area()))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);

            if let Some(action) = app.should_exit {
                return Ok(action);
            }
        }
    }
}
