use image::{DynamicImage, codecs::png::FilterType};
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;
use ratatui_image::{FilterType::Lanczos3, Image, Resize, picker::Picker};

pub struct TestImageWidget {
    pub image: DynamicImage,
}

impl Default for TestImageWidget {
    fn default() -> Self {
        let dyn_image = image::ImageReader::open("var/test.jpg")
            .unwrap()
            .decode()
            .unwrap();

        TestImageWidget { image: dyn_image }
    }
}

impl Widget for TestImageWidget {
    fn render(self, area: ratatui::layout::Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let picker = Picker::from_query_stdio().unwrap();
        let protocol = picker
            .new_protocol(self.image.clone(), area, Resize::Scale(Some(Lanczos3)))
            .unwrap();
        let image = Image::new(&protocol);
        image.render(area, buf);
    }
}
