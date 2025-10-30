mod widgets;

use widgets::{CounterWidget, DescriptionWidget, TestImageWidget};

use std::{io, process::exit, vec};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Padding, Paragraph, Widget},
};

#[derive(Default)]
struct App {
    exit: bool,
    rotation: usize,
    counter: CounterWidget,
    image: TestImageWidget,
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
                    KeyCode::Up => self.scroll_up_description(),
                    KeyCode::Down => self.scroll_down_description(),
                    _ => self.counter.handle_counter_change(key_event),
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn scroll_up_description(&mut self) {}

    fn scroll_down_description(&mut self) {}

    fn exit(&mut self) {
        self.exit = true;
    }

    fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 2;
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let full_screen = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(area);

        let description_area = Layout::vertical(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(60),
            Constraint::Percentage(15),
        ])
        .split(full_screen[0]);

        let test_image = TestImageWidget::default();

        let _image_area_width = description_area[0].width;
        let _image_area_height = description_area[0].height;
        let _image_width = test_image.image.width();
        let _image_height = test_image.image.height();

        let image_area = center(
            description_area[0],
            Constraint::Length(test_image.image.width().try_into().unwrap()),
            Constraint::Length(test_image.image.height().try_into().unwrap()),
        );
        test_image.render(image_area, buf);

        DescriptionWidget::default().render(description_area[1], buf);
        Paragraph::default()
            .block(
                Block::bordered()
                    .gray()
                    .title("Properties")
                    .padding(Padding::uniform(1)),
            )
            .render(description_area[2], buf);
        Paragraph::default()
            .block(
                Block::bordered()
                    .gray()
                    .title("Field")
                    .padding(Padding::uniform(1)),
            )
            .render(full_screen[1], buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
