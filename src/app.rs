use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
};
use std::collections::HashMap;

pub struct App {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub should_exit: Option<ShouldPrint>, // `Ok(…)` if user wants to exit; … == true iff they want to print the JSON
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

type ShouldPrint = bool;

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            should_exit: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => self.show_editing_screen(),
                    KeyCode::Char('q') => self.show_exit_screen(),
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => self.should_exit = Some(true),
                    KeyCode::Char('n') | KeyCode::Char('q') => self.should_exit = Some(false),
                    _ => {}
                },

                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => self.start_editing_value(),
                                CurrentlyEditing::Value => self.confirm_pair(),
                            }
                        }
                    }
                    KeyCode::Esc => self.abort_editing(),
                    KeyCode::Tab => self.toggle_editing(),
                    KeyCode::Char(value) => self.type_char(value),
                    KeyCode::Backspace => self.backspace_content(),
                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        match &self.currently_editing {
            Some(edit_mode) => match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            },
            _ => self.currently_editing = Some(CurrentlyEditing::Key),
        };
    }

    pub fn abort_editing(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }

    pub fn type_char(&mut self, value: char) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Key => self.key_input.push(value),
                CurrentlyEditing::Value => self.value_input.push(value),
            };
        }
    }

    pub fn backspace_content(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Key => self.key_input.pop(),
                CurrentlyEditing::Value => self.value_input.pop(),
            };
        };
    }

    pub fn show_editing_screen(&mut self) {
        self.current_screen = CurrentScreen::Editing;
        self.currently_editing = Some(CurrentlyEditing::Key);
    }

    pub fn show_exit_screen(&mut self) {
        self.current_screen = CurrentScreen::Exiting;
    }

    pub fn start_editing_value(&mut self) {
        self.currently_editing = Some(CurrentlyEditing::Value)
    }

    pub fn confirm_pair(&mut self) {
        self.save_key_value();
        self.current_screen = CurrentScreen::Main;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Create the layout sections.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        title.render(chunks[0], buf);
        let mut list_items = Vec::<ListItem>::new();

        for key in self.pairs.keys() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25} : {}", key, self.pairs.get(key).unwrap()),
                Style::default().fg(Color::Yellow),
            ))));
        }

        let list = List::new(list_items);

        list.render(chunks[1], buf);
        let current_navigation_text = vec![
            // The first half of the text
            match self.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(Color::Green))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
                }
                CurrentScreen::Exiting => {
                    Span::styled("Exiting", Style::default().fg(Color::LightRed))
                }
            }
            .to_owned(),
            // A white divider bar to separate the two sections
            Span::styled(" | ", Style::default().fg(Color::White)),
            // The final section of the text, with hints on what the user is editing
            {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];

        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));

        let current_keys_hint = {
            match self.current_screen {
                CurrentScreen::Main => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Editing => Span::styled(
                    "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
            }
        };

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        mode_footer.render(footer_chunks[0], buf);
        key_notes_footer.render(footer_chunks[1], buf);

        if let Some(editing) = &self.currently_editing {
            let popup_block = Block::default()
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = centered_rect(60, 25, area);
            popup_block.render(area, buf);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            match editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_style),
            };

            let key_text = Paragraph::new(self.key_input.clone()).block(key_block);
            key_text.render(popup_chunks[0], buf);

            let value_text = Paragraph::new(self.value_input.clone()).block(value_block);
            value_text.render(popup_chunks[1], buf);
        }

        if let CurrentScreen::Exiting = self.current_screen {
            Clear.render(area, buf); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the buffer as json? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, area);
            exit_paragraph.render(area, buf);
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
