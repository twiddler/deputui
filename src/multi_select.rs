use std::cmp;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

pub struct MultiSelectView<'a> {
    pub multi_select: &'a MultiSelect,
    pub focused: bool,
    pub block: Block<'a>,
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
                    indicator(self.focused)
                } else {
                    no_indicator()
                };

                create_option_item(&option.value, option.selected, indicator)
            })
            .map(|o| ListItem::new(o))
            .collect();

        List::new(list_items).block(self.block).render(area, buf);
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

        self.cursor = cmp::min(max_cursor, self.cursor.saturating_add(1))
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

    pub fn focused_value(&self) -> String {
        self.options[self.cursor].value.clone()
    }
}

struct SelectOption {
    value: String,
    selected: bool,
}

fn indicator(focused: bool) -> Span<'static> {
    Span::styled(
        ">",
        match focused {
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
