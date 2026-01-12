use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::{app_shell::AppShell, multi_select::MultiSelectView};
use crate::{
    async_h1_client::get,
    multi_select::{MultiSelect, SelectOption},
};

const SCROLL_STEP_SIZE: u16 = 5;

pub struct App {
    scroll: u16,
    release_notes: Option<String>,
    focused_pane: Pane,
    multiselect: MultiSelect<Release>,
    pub should_exit: Option<ExitAction>, // `Ok(…)` if user wants to exit; … == true iff they want to print the selected releases
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
    pub fn new(releases: &[Release]) -> App {
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

        let mut app = App {
            scroll: 0,
            multiselect: MultiSelect::new(options),
            release_notes: None,
            focused_pane,
            should_exit: None,
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
        // let release = self.multiselect.focused_value();
        // self.release_notes = get_release_notes_of(release);
        let url = "https://example.com";

        self.release_notes = match smol::block_on(get(url)) {
            Ok(s) => Some(s),
            Err(e) => Some(format!("--- Error fetching release notes: {e} ---")),
        };
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
}

fn get_release_notes_of(release: &Release) -> Option<String> {
    match release.to_string().as_str() {
            "foo@1.0.0" => Some(
                "# Level 1\n\
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
            *Lorem ipsum* dolor sit amet, consectetur adipiscing elit. Morbi molestie nisi eros, ut viverra enim finibus id. Integer vitae lacus sit amet nisl eleifend malesuada at quis purus. Nulla cursus dignissim nisi, ut imperdiet ipsum aliquet a. Cras ultrices dignissim ultricies. Pellentesque sit amet blandit tortor, id porta felis. In hac habitasse platea dictumst. Praesent id leo risus. Etiam porttitor tellus neque, in laoreet tellus malesuada at. Duis placerat ultricies vehicula. Sed commodo nisi et tempor convallis. In volutpat ipsum eget ex sodales dictum.".to_string(),
            ),
            "bar@2.2.2" => Some("bar".to_string()),
            _ => None,
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
        let release_notes_text = match &self.release_notes {
            None => Text::styled("--- No release notes ---", Color::Yellow),
            Some(s) => tui_markdown::from_str(s),
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
        }
        .render(area, buf);
    }
}

fn get_keys_hints(pane: &Pane) -> &'static str {
    match pane {
        Pane::Releases => {
            "down: j | up: k | focus release notes: l | toggle: ␣ | confirm: ⏎ | abort: ctrl+c"
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

// Release
//
#[derive(Clone, Eq, PartialEq)]
pub struct Release {
    pub package: String,
    pub semver: String,
}

impl ToString for Release {
    fn to_string(&self) -> String {
        format!("{}@{}", self.package, self.semver)
    }
}

impl Ord for Release {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.package.cmp(&other.package) {
            std::cmp::Ordering::Equal => self.semver.cmp(&other.semver),
            other => other,
        }
    }
}

impl PartialOrd for Release {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
