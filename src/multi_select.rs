use std::cmp;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Widget},
};

pub struct MultiSelectView<'a> {
    pub multi_select: &'a MultiSelect,
    pub active: bool,
}

impl Widget for MultiSelectView<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        let list_items: Vec<ListItem> = self
            .multi_select
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let indicator = if i == self.multi_select.cursor {
                    indicator(self.active)
                } else {
                    no_indicator()
                };

                create_option_item(&option.value, option.selected, indicator)
            })
            .map(|o| ListItem::new(o))
            .collect();

        List::new(list_items)
            .block(get_block(self.active))
            .render(area, buf);
    }
}

pub struct MultiSelect {
    options: Vec<SelectOption>,
    cursor: usize,
}

impl MultiSelect {
    pub fn new(values: &[&str]) -> MultiSelect {
        MultiSelect {
            options: values
                .iter()
                .map(|&s| SelectOption {
                    value: s.into(),
                    selected: false,
                })
                .collect(),
            cursor: 0,
        }
    }

    pub fn previous(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn next(&mut self) {
        let max_cursor = self.options.len() - 1;

        self.cursor = cmp::min(max_cursor, self.cursor.wrapping_add(1))
    }

    pub fn toggle(&mut self) {
        self.options[self.cursor].selected = !self.options[self.cursor].selected;
    }

    pub fn selected(&self) -> Vec<&str> {
        self.options
            .iter()
            .filter(|o| o.selected)
            .map(|o| o.value.as_str())
            .collect()
    }
}

struct SelectOption {
    value: String,
    selected: bool,
}

fn indicator(active: bool) -> Span<'static> {
    Span::styled(
        ">",
        match active {
            true => Style::default().fg(Color::Cyan),
            false => Style::default().fg(Color::DarkGray),
        },
    )
}

fn no_indicator() -> Span<'static> {
    Span::raw(" ")
}

fn create_option_item<'a>(label: &'a str, selected: bool, indicator: Span<'a>) -> Line<'a> {
    Line::from(vec![
        indicator,
        Span::styled("[", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if selected { "x" } else { " " },
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("]", Style::default().fg(Color::DarkGray)),
        Span::raw(format!(" {: <25}", label)),
    ])
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
