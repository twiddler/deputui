use std::cmp;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

pub struct MultiSelectView<'a, T> {
    pub multi_select: &'a MultiSelect<T>,
    pub focused: bool,
    pub block: Block<'a>,
}

impl<T> Widget for MultiSelectView<'_, T> {
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
            .map(|(i, select_option)| {
                let indicator = if i == self.multi_select.cursor {
                    indicator(self.focused)
                } else {
                    no_indicator()
                };

                create_option_item(&select_option.label, select_option.selected, indicator)
            })
            .map(|o| ListItem::new(o))
            .collect();

        List::new(list_items).block(self.block).render(area, buf);
    }
}

pub struct MultiSelect<T> {
    options: Vec<SelectOption<T>>,
    cursor: usize,
}

impl<T> MultiSelect<T> {
    pub fn new(options: Vec<SelectOption<T>>) -> MultiSelect<T> {
        MultiSelect { options, cursor: 0 }
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

    pub fn selected_values(&self) -> Vec<&T> {
        self.options
            .iter()
            .filter(|o| o.selected)
            .map(|o| &o.value)
            .collect()
    }

    pub fn focused_value(&self) -> &T {
        &self.options[self.cursor].value
    }
}

pub struct SelectOption<T> {
    label: String,
    value: T,
    selected: bool,
}

impl<T> SelectOption<T> {
    pub fn new(label: String, value: T) -> SelectOption<T> {
        SelectOption {
            label,
            value,
            selected: false,
        }
    }
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
            Style::default().fg(Color::Green),
        ),
        Span::styled("]", Style::default().fg(Color::DarkGray)),
        Span::raw(format!(" {: <25}", label)),
    ])
}
