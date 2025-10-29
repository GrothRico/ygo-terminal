use ratatui::buffer::Buffer;
use ratatui::prelude::StatefulWidget;
use ratatui::widgets::Widget;
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

pub struct TestImageWidget {
    pub image: StatefulProtocol,
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
    fn render(self, area: ratatui::layout::Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let image_widget = StatefulImage::default();
        image_widget.render(area, buf, &mut self.image);
    }
}
