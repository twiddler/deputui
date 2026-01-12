use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, poll},
};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    error::Error,
    io::{self, Read},
    time::Duration,
};

mod app;
mod app_shell;
mod async_h1_client;
mod async_task;
mod jobs;
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

    // Use smol::block_on to run async processing in sync context
    loop {
        // Process async events using smol
        let _has_events = smol::block_on(async {
            app.process_async_events().await;
            app.async_task.is_running()
        });

        terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;

        // Use non-blocking event reading with timeout
        if poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);

                if let Some(action) = &app.should_exit {
                    let result = action == &ExitAction::PrintSelected;
                    restore_terminal(&mut terminal)?;

                    if result {
                        let output = app
                            .get_selected_releases()
                            .iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        println!("{output}");
                    } else {
                        return Err("Not printing; aborting with exit code 1 …".into());
                    }
                }
            }
        }
    }
}

// Input parsing remains the same
#[derive(Deserialize)]
struct PnpmOutdatedPackage {
    wanted: String,
}

type PnpmOutdatedOutput = BTreeMap<String, PnpmOutdatedPackage>;

fn parse_input() -> Result<PnpmOutdatedOutput, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}