use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

pub struct AppShell<L, R, F> {
    pub left: L,
    pub right: R,
    pub footer: F,
}

impl<L: Widget, R: Widget, F: Widget> Widget for AppShell<L, R, F> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

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

        self.left.render(left_column, buf);
        self.right.render(right_column, buf);
        self.footer.render(footer_chunk, buf);
    }
}
