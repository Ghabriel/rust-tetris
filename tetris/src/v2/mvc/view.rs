use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use super::Model;
use super::traits::Render;

pub struct View {
    window: RenderWindow,
}

impl Render for View {
    type Target = Model;

    fn render(&mut self, model: &Model) -> bool {
        let window = &mut self.window;

        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
                return true;
            }
        }

        window.set_active(true);
        // window.draw(&shape);
        window.display();

        false
    }
}

impl View {
    pub fn new(
        width: u32,
        height: u32,
        title: &str
    ) -> View {
        let window = RenderWindow::new(
            (width, height),
            title,
            Style::CLOSE,
            &Default::default()
        );

        View {
            window,
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
