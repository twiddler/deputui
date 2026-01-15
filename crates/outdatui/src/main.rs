use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyEvent},
    Terminal,
};
use smol::{block_on, channel, future, Executor};
use std::{
    error::Error,
    io::{self, Read},
};

mod app;
mod app_shell;
mod async_task;
mod github;
mod multi_select;
mod release_ext;
mod tui;
use crate::app::{App, ExitAction};
use crate::tui::{restore_terminal, setup_terminal};
use common::release::Release;

fn main() -> Result<(), Box<dyn Error>> {
    let mut releases: Vec<Release> = parse_json_input()?;

    releases.sort();

    let mut terminal = setup_terminal()?;

    // Create channel for render triggers
    let (data_ready_tx, data_ready_rx) = channel::bounded::<()>(1);

    let mut app = App::new(&releases, data_ready_tx);
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

fn parse_json_input() -> Result<Vec<Release>, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
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
                // Channel signal received - task completion notification
                // The next loop iteration will re-render with updated state
            }

            EventSource::Unhandled => {}

            EventSource::Error(e) => return Err(e.into()),
        }
    }
}
