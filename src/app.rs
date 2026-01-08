use std::cmp;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget},
};

const SCROLL_STEP_SIZE: u16 = 5;

pub struct App<'a> {
    pub cursor: usize,
    pub scroll: u16,
    pub releases: &'a [&'a str],
    pub release_notes: &'a str,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub should_exit: Option<ShouldPrint>, // `Ok(…)` if user wants to exit; … == true iff they want to print the JSON
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Releases,
    ReleaseNotes,
}

type ShouldPrint = bool;

impl<'a> App<'a> {
    pub fn new(foo: &'a [&str]) -> App<'a> {
        App {
            cursor: 0,
            scroll: 0,
            releases: foo,
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
            current_screen: CurrentScreen::Releases,
            should_exit: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match self.current_screen {
                CurrentScreen::Releases => match key.code {
                    KeyCode::Char('l') => self.focus_release_notes(),

                    KeyCode::Char('k') => self.focus_previous_option(),
                    KeyCode::Char('j') => self.focus_next_option(),

                    KeyCode::Enter => self.should_exit = Some(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_exit = Some(false)
                    }
                    _ => {}
                },

                CurrentScreen::ReleaseNotes if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('h') => self.focus_releases(),

                    KeyCode::Char('k') => self.scroll_up(),
                    KeyCode::Char('j') => self.scroll_down(),

                    KeyCode::Enter => self.should_exit = Some(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_exit = Some(false)
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn focus_previous_option(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn focus_next_option(&mut self) {
        let max_cursor = self.releases.len() - 1;

        self.cursor = cmp::min(max_cursor, self.cursor.wrapping_add(1))
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(SCROLL_STEP_SIZE)
    }

    pub fn scroll_down(&mut self) {
        let max_line = u16::try_from(self.releases.len()).unwrap();

        self.scroll = cmp::min(max_line, self.scroll.wrapping_add(SCROLL_STEP_SIZE));
    }

    pub fn focus_releases(&mut self) {
        self.current_screen = CurrentScreen::Releases;
    }

    pub fn focus_release_notes(&mut self) {
        self.current_screen = CurrentScreen::ReleaseNotes;
    }

    pub fn get_selection(&self) -> String {
        self.releases.join(" ")
    }
}

fn get_block<'a>(active: bool) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_style(active))
        .style(Style::default())
}

fn get_style<'a>(active: bool) -> Style {
    match active {
        true => Style::default(),
        false => Style::default().fg(Color::DarkGray),
    }
}

fn layout(
    left: List,
    right: Paragraph,
    footer: Span,
    area: Rect,
    buf: &mut ratatui::buffer::Buffer,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_chunk = chunks[0];
    let footer_chunk = chunks[1];

    let column_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(1)])
        .split(main_chunk);

    let left_column = column_chunks[0];
    let right_column = column_chunks[1];

    left.render(left_column, buf);
    right.render(right_column, buf);
    footer.render(footer_chunk, buf);
}

fn indicator<'a>(active: bool) -> Span<'a> {
    Span::styled(">", get_style(active))
}

fn no_indicator() -> &'static str {
    " "
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let release_notes = Paragraph::new(tui_markdown::from_str(self.release_notes))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((self.scroll, 0))
            .block(get_block(
                self.current_screen == CurrentScreen::ReleaseNotes,
            ));

        let mut list_items = Vec::<ListItem>::new();

        for (i, release) in self.releases.iter().enumerate() {
            list_items.push(ListItem::new(Line::from(vec![
                if i == self.cursor {
                    indicator(self.current_screen == CurrentScreen::Releases).into()
                } else {
                    no_indicator().into()
                },
                format!(" [{}] {: <25}", " ", release).into(),
            ])));
        }

        let releases =
            List::new(list_items).block(get_block(self.current_screen == CurrentScreen::Releases));

        let keys_hints = Span::styled(
            get_keys_hints(&self.current_screen),
            Style::default().fg(Color::DarkGray),
        );

        layout(releases, release_notes, keys_hints, area, buf);
    }
}

fn get_keys_hints(screen: &CurrentScreen) -> &'static str {
    match screen {
        CurrentScreen::Releases => {
            "down: j | up: k | focus release notes: l | toggle: space | confirm: enter | abort: ctrl+c"
        }
        CurrentScreen::ReleaseNotes => {
            "down: j | up: k | focus packages: h | confirm: enter | abort: ctrl+c"
        }
    }
}
