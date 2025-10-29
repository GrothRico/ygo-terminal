use std::vec;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{
        Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget, Wrap,
    },
};

#[derive(Default)]
pub struct VerticalScrollbarState {
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
}

impl VerticalScrollbarState {
    fn increase(&mut self) {
        let _ = self.vertical_scroll.saturating_add(1);
        let _ = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    fn decrease(&mut self) {
        let _ = self.vertical_scroll.saturating_sub(1);
        let _ = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    fn set_content_length(&mut self, lines: Vec<Line<'_>>) {
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(lines.len());
    }
}

#[derive(Default)]
pub struct DescriptionWidget {
    pub vertical_scroll_state: VerticalScrollbarState,
}

impl Widget for DescriptionWidget {
    fn render(mut self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text = vec![
            Line::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
            Line::from("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".red()),
            Line::from("Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.".on_dark_gray()),
            Line::from("Nisi ut aliquip ex ea commodo consequat.".crossed_out()),
            Line::from("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".reset()),
            Line::from(vec![
                Span::raw("Masked lorem: "),
                Span::styled(Masked::new("dolor sit amet", '*'), Style::new().fg(Color::Blue)),
            ]),
            Line::from("Excepteur sint occaecat cupidatat non proident.".red()),
            Line::from("Sunt in culpa qui officia deserunt mollit anim id est laborum.".on_dark_gray()),
            Line::from("Curabitur pretium tincidunt lacus. Nulla gravida orci a odio.".crossed_out()),
            Line::from(vec![
                Span::raw("Masked secret: "),
                Span::styled(Masked::new("loremipsum", '*'), Style::new().fg(Color::Yellow)),
            ]),
        ];
        let text_block = Block::bordered()
            .gray()
            .title("Description")
            .padding(Padding::uniform(1));
        Paragraph::new(text.clone())
            .wrap(Wrap { trim: false })
            .block(text_block)
            .scroll((self.vertical_scroll_state.vertical_scroll as u16, 0))
            .render(area, buf);

        self.vertical_scroll_state.set_content_length(text);
        Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
            area,
            buf,
            &mut self.vertical_scroll_state.vertical_scroll_state,
        );
    }
}
