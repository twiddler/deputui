use crossterm::event::{Event, EventStream, KeyEvent};
use futures::StreamExt;
use ratatui::{backend::Backend, Terminal};
use smol::{block_on, channel, Executor};
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

#[derive(Debug)]
pub enum UiMessage {
    Key(KeyEvent),
    TaskComplete,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut releases: Vec<Release> = parse_json_input()?;

    releases.sort();

    let mut terminal = setup_terminal()?;

    // Create unified event channel
    let (ui_tx, ui_rx) = channel::unbounded::<UiMessage>();

    // Spawn keyboard event reader task that sends through unified channel
    let keyboard_ui_tx = ui_tx.clone();
    smol::spawn(async move {
        let mut reader = EventStream::new();
        while let Some(result) = reader.next().await {
            match result {
                Ok(Event::Key(key)) => {
                    keyboard_ui_tx.send(UiMessage::Key(key)).await.ok();
                }
                Ok(_) => {
                    // Ignore non-key events
                }
                Err(_) => {
                    // Stream error, break the loop
                    break;
                }
            }
        }
    })
    .detach();

    let mut app = App::new(&releases, ui_tx);
    let res = block_on(async {
        let executor = Executor::new();
        executor
            .run(run_app_async(&mut terminal, &mut app, ui_rx))
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
    ui_rx: smol::channel::Receiver<UiMessage>,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| frame.render_widget(&*app, frame.area()))?;

        match ui_rx.recv().await {
            Ok(UiMessage::Key(key)) => {
                app.handle_key(key);

                if let Some(action) = &app.should_exit {
                    return Ok(action == &ExitAction::PrintSelected);
                }
            }

            Ok(UiMessage::TaskComplete) => {
                // Task completion notification received
                // The next loop iteration will re-render with updated state
            }

            Err(_) => {
                // Channel closed, exit with error
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "UI channel closed",
                ));
            }
        }
    }
}
