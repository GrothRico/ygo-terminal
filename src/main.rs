use std::{io, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Masked, Span, Text},
    widgets::{
        Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget, Wrap,
    },
};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

#[derive(Default)]
struct App {
    exit: bool,
    rotation: usize,
    counter: CounterWidget,
    image: TestImageWidget,
}

#[derive(Default)]
struct CounterWidget {
    counter: u8,
}

impl CounterWidget {
    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn handle_counter_change(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
}

impl Widget for &CounterWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

struct TestImageWidget {
    image: StatefulProtocol,
}

impl Default for TestImageWidget {
    fn default() -> Self {
        let picker = Picker::from_query_stdio().unwrap();
        let dyn_image = image::ImageReader::open("var/test.jpg")
            .unwrap()
            .decode()
            .unwrap();
        TestImageWidget {
            image: picker.new_resize_protocol(dyn_image),
        }
    }
}

impl Widget for &mut TestImageWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let image_widget = StatefulImage::default();
        image_widget.render(area, buf, &mut self.image);
    }
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('r') => self.rotate(),
                    _ => self.counter.handle_counter_change(key_event),
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 2;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);
        // TODO: https://ratatui.rs/examples/widgets/scrollbar/
        DescriptionWidget::default().render(layout[0], buf);
        Paragraph::default()
            .block(
                Block::bordered()
                    .gray()
                    .title("Field")
                    .padding(Padding::uniform(1)),
            )
            .render(layout[1], buf);
    }
}

#[derive(Default)]
struct VerticalScrollbarState {
    vertical_scroll_state: ScrollbarState,
    vertical_scroll: usize,
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
struct DescriptionWidget {
    vertical_scroll_state: VerticalScrollbarState,
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

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
