use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use super::Model;

pub struct View<'a> {
    model: &'a Model,
    window: RenderWindow,
}

impl<'a> View<'a> {
    pub fn new<'b>(
        model: &'b Model,
        width: u32,
        height: u32,
        title: &str
    ) -> View<'b> {
        let window = RenderWindow::new(
            (width, height),
            title,
            Style::CLOSE,
            &Default::default()
        );

        View {
            window,
            model,
        }
    }

    pub fn init(&mut self) {
        let window = &mut self.window;
        let shape = CircleShape::new(100., 30);

        while window.is_open() {
            while let Some(event) = window.poll_event() {
                if event == Event::Closed {
                    window.close();
                }
            }

            window.set_active(true);
            window.draw(&shape);
            window.display();
        }
    }
}
