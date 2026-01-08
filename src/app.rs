use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app_shell::AppShell;
use crate::multi_select::MultiSelect;

const SCROLL_STEP_SIZE: u16 = 5;

pub struct App<'a> {
    scroll: u16,
    release_notes: &'a str,
    focused_pane: Pane,
    multiselect: MultiSelect,
    pub should_exit: Option<ShouldPrint>, // `Ok(…)` if user wants to exit; … == true iff they want to print the JSON
}

#[derive(PartialEq)]
pub enum Pane {
    Releases,
    ReleaseNotes,
}

type ShouldPrint = bool;

impl<'a> App<'a> {
    pub fn new(releases: &'a [&str]) -> App<'a> {
        let focused_pane = Pane::Releases;

        App {
            scroll: 0,
            multiselect: MultiSelect::new(releases, focused_pane == Pane::Releases),
            release_notes: "# Level 1\n\
\n\
            **Lorem ipsum dolor sit amet**, consectetur adipiscing elit. Morbi molestie nisi eros, ut viverra enim finibus id. Integer vitae lacus sit amet nisl eleifend malesuada at quis purus. Nulla cursus dignissim nisi, ut imperdiet ipsum aliquet a. Cras ultrices dignissim ultricies. Pellentesque sit amet blandit tortor, id porta felis. In hac habitasse platea dictumst. Praesent id leo risus. Etiam porttitor tellus neque, in laoreet tellus malesuada at. Duis placerat ultricies vehicula. Sed commodo nisi et tempor convallis. In volutpat ipsum eget ex sodales dictum.\n\
\n\
# Level 2\n\
\n\
            **Lorem ipsum dolor sit amet**, consectetur adipiscing elit. Morbi molestie nisi eros, ut viverra enim finibus id. Integer vitae lacus sit amet nisl eleifend malesuada at quis purus. Nulla cursus dignissim nisi, ut imperdiet ipsum aliquet a. Cras ultrices dignissim ultricies. Pellentesque sit amet blandit tortor, id porta felis. In hac habitasse platea dictumst. Praesent id leo risus. Etiam porttitor tellus neque, in laoreet tellus malesuada at. Duis placerat ultricies vehicula. Sed commodo nisi et tempor convallis. In volutpat ipsum eget ex sodales dictum.\n\
\n\
# Level 3\n\
\n\
            **Lorem ipsum dolor sit amet**, consectetur adipiscing elit. Morbi molestie nisi eros, ut viverra enim finibus id. Integer vitae lacus sit amet nisl eleifend malesuada at quis purus. Nulla cursus dignissim nisi, ut imperdiet ipsum aliquet a. Cras ultrices dignissim ultricies. Pellentesque sit amet blandit tortor, id porta felis. In hac habitasse platea dictumst. Praesent id leo risus. Etiam porttitor tellus neque, in laoreet tellus malesuada at. Duis placerat ultricies vehicula. Sed commodo nisi et tempor convallis. In volutpat ipsum eget ex sodales dictum.\n\
\n\
            ## Something deeper\n\
\n\
            *Lorem ipsum* dolor sit amet, consectetur adipiscing elit. Morbi molestie nisi eros, ut viverra enim finibus id. Integer vitae lacus sit amet nisl eleifend malesuada at quis purus. Nulla cursus dignissim nisi, ut imperdiet ipsum aliquet a. Cras ultrices dignissim ultricies. Pellentesque sit amet blandit tortor, id porta felis. In hac habitasse platea dictumst. Praesent id leo risus. Etiam porttitor tellus neque, in laoreet tellus malesuada at. Duis placerat ultricies vehicula. Sed commodo nisi et tempor convallis. In volutpat ipsum eget ex sodales dictum.",
            focused_pane,
            should_exit: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match self.focused_pane {
                Pane::Releases => match key.code {
                    KeyCode::Char('l') => self.focus_release_notes(),
                    KeyCode::Char('k') => self.multiselect.previous(),
                    KeyCode::Char('j') => self.multiselect.next(),
                    KeyCode::Char(' ') => self.multiselect.toggle(),
                    KeyCode::Enter => self.should_exit = Some(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_exit = Some(false)
                    }
                    _ => {}
                },

                Pane::ReleaseNotes => match key.code {
                    KeyCode::Char('h') => self.focus_releases(),
                    KeyCode::Char('k') => self.scroll_up(),
                    KeyCode::Char('j') => self.scroll_down(),
                    KeyCode::Enter => self.should_exit = Some(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_exit = Some(false)
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(SCROLL_STEP_SIZE)
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.wrapping_add(SCROLL_STEP_SIZE);
    }

    pub fn focus_releases(&mut self) {
        self.focused_pane = Pane::Releases;
        self.multiselect.active = true;
    }

    pub fn focus_release_notes(&mut self) {
        self.focused_pane = Pane::ReleaseNotes;
        self.multiselect.active = false;
    }

    pub fn get_selected_releases(&self) -> String {
        self.multiselect.selected().join(" ")
    }
}

fn get_style<'a>(active: bool) -> Style {
    match active {
        true => Style::default(),
        false => Style::default().fg(Color::DarkGray),
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let release_notes = Paragraph::new(tui_markdown::from_str(self.release_notes))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((self.scroll, 0))
            .block(get_block(self.focused_pane == Pane::ReleaseNotes));

        let keys_hints = Line::styled(
            get_keys_hints(&self.focused_pane),
            Style::default().fg(Color::DarkGray),
        )
        .centered();

        AppShell {
            left: &self.multiselect,
            right: release_notes,
            footer: keys_hints,
        }
        .render(area, buf);
    }
}

fn get_keys_hints(pane: &Pane) -> &'static str {
    match pane {
        Pane::Releases => {
            "down: j | up: k | focus release notes: l | toggle: space | confirm: enter | abort: ctrl+c"
        }
        Pane::ReleaseNotes => {
            "down: j | up: k | focus packages: h | confirm: enter | abort: ctrl+c"
        }
    }
}

fn get_block<'a>(active: bool) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_style(active))
        .style(Style::default())
}
