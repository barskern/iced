//! Showcasing image clipping on the canvas.
use iced::advanced::image::Renderer as IRenderer;
use iced::mouse;
use iced::widget::canvas::Geometry;
use iced::widget::{canvas, image};
use iced::{color, Element, Fill, Point, Rectangle, Renderer, Size};
use tracing::debug;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(
        "Image Clipping - Iced",
        ImageClipping::update,
        ImageClipping::view,
    )
    .run()
}

#[derive(Default)]
struct ImageClipping {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl ImageClipping {
    fn update(&mut self, _message: Message) {}
    fn view(&self) -> Element<Message> {
        canvas(&self.state).width(Fill).height(Fill).into()
    }
}

#[derive(Debug)]
struct State {
    sun: image::Handle,
    cache: canvas::Cache,
}

impl State {
    pub fn new() -> State {
        State {
            sun: image::Handle::from_bytes(
                include_bytes!("../../solar_system/assets/sun.png").as_slice(),
            ),
            cache: Default::default(),
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {

        let result = self.cache.draw(renderer, bounds.size(), |frame| {
            let image_size = {
                let s = renderer.measure_image(&self.sun);
                Size::new(s.width as f32, s.height as f32)
            };

            let image_rect = Rectangle::with_size(image_size);
            // Image should be halved in the horizontal direction
            let mut clipping_rect = Rectangle::with_size(Size::new(image_size.width / 2.0, image_size.height));

            debug!("Clipping Area: {:?}", clipping_rect);
            debug!("Image Area: {:?}", image_rect);

            frame.with_clip(clipping_rect, |clipped_frame| {
                // This rectangle with the same dimensions as the image is clipped correctly
                clipped_frame.fill_rectangle(
                    Point::ORIGIN,
                    image_size,
                    color!(0xFF0000),
                );
            });

            // Move the clipping area "down" to draw the examples at the same time
            clipping_rect.y += image_size.height;

            frame.with_clip(clipping_rect, |clipped_frame| {
                // This image is not clipped/cropped to the clipping_rect area and is overflowing.
                clipped_frame.draw_image(image_rect, &self.sun);
            });
        });

        vec![result]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
