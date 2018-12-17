use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use super::{Controller, EventListener};

pub struct View {
    controller: Controller,
    window: RenderWindow,
}

impl View {
    pub fn new(
        controller: Controller,
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
            controller,
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

impl EventListener for View {}