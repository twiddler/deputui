use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use smol::channel::Sender;

use crate::async_task::{AsyncTask, AsyncTaskStatus};
use crate::multi_select::{MultiSelect, SelectOption};
use crate::{app_shell::AppShell, multi_select::MultiSelectView, release_ext::ReleaseExt};
use common::release::Release;

const SCROLL_STEP_SIZE: u16 = 5;

pub struct App {
    scroll: u16,
    focused_pane: Pane,
    multiselect: MultiSelect<Release>,
    pub should_exit: Option<ExitAction>, // `Ok(…)` if user wants to exit; … == true iff they want to print the selected releases
    left_column_width: u16,
    release_notes_task: AsyncTask<String, Release>,
}

#[derive(PartialEq)]
pub enum Pane {
    Releases,
    ReleaseNotes,
}

#[derive(PartialEq)]
pub enum ExitAction {
    Abort,
    PrintSelected,
}

impl App {
    pub fn new(releases: &[Release], render_tx: Sender<()>) -> App {
        let focused_pane = Pane::Releases;

        let options = releases
            .iter()
            .map(|release| {
                SelectOption::new(
                    format!("{}@{}", release.package, release.semver),
                    release.clone(),
                )
            })
            .collect();

        let release_notes_task = AsyncTask::new(
            |release: Release| {
                smol::block_on(async move { ReleaseExt(&release).fetch_release_notes().await })
            },
            render_tx.clone(),
        );

        let mut app = App {
            scroll: 0,
            multiselect: MultiSelect::new(options),
            focused_pane,
            should_exit: None,
            left_column_width: 40,
            release_notes_task,
        };

        app.show_release_notes_of_focused_release();

        app
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_exit = Some(ExitAction::Abort);
            return;
        }

        match self.focused_pane {
            Pane::Releases => match key.code {
                KeyCode::Char('l') => self.focus_pane(Pane::ReleaseNotes),
                KeyCode::Char('k') => {
                    self.multiselect.previous();
                    self.show_release_notes_of_focused_release();
                }
                KeyCode::Char('j') => {
                    self.multiselect.next();
                    self.show_release_notes_of_focused_release();
                }
                KeyCode::Char('-') => self.shrink_left_column(),
                KeyCode::Char('+') => self.expand_left_column(),
                KeyCode::Char(' ') => self.multiselect.toggle(),
                KeyCode::Enter => self.should_exit = Some(ExitAction::PrintSelected),
                _ => {}
            },

            Pane::ReleaseNotes => match key.code {
                KeyCode::Char('h') => self.focus_pane(Pane::Releases),
                KeyCode::Char('k') => self.scroll_up(),
                KeyCode::Char('j') => self.scroll_down(),
                _ => {}
            },
        }
    }

    pub fn show_release_notes_of_focused_release(&mut self) {
        let release = self.multiselect.focused_value();
        self.release_notes_task.start_operation(release.clone());
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(SCROLL_STEP_SIZE)
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(SCROLL_STEP_SIZE);
    }

    pub fn focus_pane(&mut self, pane: Pane) {
        self.focused_pane = pane;
    }

    pub fn get_selected_releases(&self) -> Vec<&Release> {
        self.multiselect.selected_values()
    }

    pub fn shrink_left_column(&mut self) {
        self.left_column_width = self.left_column_width.saturating_sub(1);
    }

    pub fn expand_left_column(&mut self) {
        self.left_column_width = self.left_column_width.saturating_add(1);
    }
}

fn get_style(focused: bool) -> Style {
    match focused {
        true => Style::default(),
        false => Style::default().fg(Color::DarkGray),
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let task_status = self.release_notes_task.status();
        let release_notes_text = match task_status {
            AsyncTaskStatus::Idle => Text::styled("--- No release notes ---", Color::Yellow),
            AsyncTaskStatus::Loading => {
                Text::styled("--- Loading release notes... ---", Color::Gray)
            }
            AsyncTaskStatus::Loaded(notes) => {
                // For now, use plain text instead of markdown to avoid lifetime issues
                Text::raw(notes)
            }
            AsyncTaskStatus::Error(error) => {
                Text::styled(format!("--- Error: {} ---", error), Color::Red)
            }
        };

        let release_notes = Paragraph::new(release_notes_text)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((self.scroll, 0))
            .block(get_block(self.focused_pane == Pane::ReleaseNotes));

        let keys_hints = Line::styled(
            get_keys_hints(&self.focused_pane),
            Style::default().fg(Color::DarkGray),
        )
        .centered();

        AppShell {
            left: MultiSelectView {
                multi_select: &self.multiselect,
                focused: self.focused_pane == Pane::Releases,
                block: get_block(self.focused_pane == Pane::Releases),
            },
            right: release_notes,
            footer: keys_hints,
            left_column_width: self.left_column_width,
        }
        .render(area, buf);
    }
}

fn get_keys_hints(pane: &Pane) -> &'static str {
    match pane {
        Pane::Releases => {
            "down: j | up: k | focus release notes: l | toggle: ␣ | confirm: ⏎ | abort: ctrl+c | +: grow | -: shrink"
        }
        Pane::ReleaseNotes => "down: j | up: k | focus releases: h | abort: ctrl+c",
    }
}

fn get_block(focused: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_style(focused))
        .style(Style::default())
}
