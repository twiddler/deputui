use futures::future::try_join_all;
use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyEvent},
};
use smol::{Executor, block_on, channel, future};
use std::{
    error::Error,
    io::{self},
};

mod app;
mod app_shell;
mod async_h1_client;
mod multi_select;
mod npm_registry;
mod pnpm;
mod semver;
mod tui;
use crate::npm_registry::NpmPackage;
use crate::pnpm::PnpmOutdatedOutput;
use crate::semver::Semver;
use crate::tui::{restore_terminal, setup_terminal};
use crate::{
    app::{App, ExitAction, Release},
    pnpm::parse_input,
};

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse_input()?;

    let mut releases: Vec<Release> = block_on(fetch_all_releases(parsed))?;

    releases.sort();

    let mut terminal = setup_terminal()?;

    // Create channel for render triggers
    let (_data_ready_tx, data_ready_rx) = channel::bounded::<()>(1);

    let mut app = App::new(&releases, _data_ready_tx);
    let res = block_on(async {
        let executor = Executor::new();
        executor
            .run(run_app_async(&mut terminal, &mut app, data_ready_rx))
            .await
    });

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

async fn fetch_all_releases(parsed: PnpmOutdatedOutput) -> Result<Vec<Release>, Box<dyn Error>> {
    let package_futures: Vec<_> = parsed
        .into_iter()
        .map(async |(package_name, package_info)| {
            let npm_package = NpmPackage::fetch(&package_name).await?;
            let current: Semver = package_info.current.parse()?;
            let latest: Semver = package_info.latest.parse()?;
            let releases = npm_package.fetch_releases(current, latest).await?;
            Ok::<Vec<app::Release>, Box<dyn Error>>(releases)
        })
        .collect();

    let all_releases: Vec<Release> = try_join_all(package_futures)
        .await?
        .into_iter()
        .flatten()
        .collect();

    Ok::<Vec<Release>, Box<dyn Error>>(all_releases)
}

async fn run_app_async<B: Backend<Error = io::Error>>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    data_ready_rx: smol::channel::Receiver<()>,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| frame.render_widget(&*app, frame.area()))?;

        enum EventSource {
            Key(KeyEvent),
            Channel,
            Unhandled,
            Error(io::Error),
        }

        let key_event = async {
            match event::read() {
                Ok(Event::Key(key)) => EventSource::Key(key),
                Ok(_) => EventSource::Unhandled,
                Err(e) => EventSource::Error(e),
            }
        };

        let channel_event = async {
            data_ready_rx.recv().await.ok();
            EventSource::Channel
        };

        match future::or(key_event, channel_event).await {
            EventSource::Key(key) => {
                app.handle_key(key);

                if let Some(action) = &app.should_exit {
                    return Ok(action == &ExitAction::PrintSelected);
                }
            }

            EventSource::Channel => {
                // Channel signal received - handle async data arrival
                // TODO: Implement async data refresh logic
            }

            EventSource::Unhandled => {}

            EventSource::Error(e) => return Err(e.into()),
        }
    }
}
