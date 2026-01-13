use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event},
};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    error::Error,
    io::{self, Read},
};

mod app;
mod app_shell;
mod async_h1_client;
mod multi_select;
mod tui;
use crate::app::{App, ExitAction, Release};
use crate::tui::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse_input()?;

    let mut releases: Vec<Release> = parsed
        .into_iter()
        .map(|(package_name, package_info)| Release {
            package: package_name,
            semver: package_info.wanted,
        })
        .collect();

    releases.sort();

    let mut terminal = setup_terminal()?;

    let mut app = App::new(&releases);
    let res = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    match res {
        Ok(true) => {
            let output = app
                .get_selected_releases()
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ");
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

            if let Some(action) = &app.should_exit {
                return Ok(action == &ExitAction::PrintSelected);
            }
        }
    }
}

// Input
//
#[derive(Deserialize)]
struct PnpmOutdatedPackage {
    current: String,
    wanted: String,
}

type PnpmOutdatedOutput = BTreeMap<String, PnpmOutdatedPackage>;

fn parse_input() -> Result<PnpmOutdatedOutput, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}
