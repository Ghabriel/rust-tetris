use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use super::{Controller, EventListener};

pub struct View<'a> {
    controller: Controller<'a>,
    window: RenderWindow,
}

impl<'a> View<'a> {
    pub fn new<'b>(
        controller: Controller<'b>,
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
            controller,
            window,
        }
    }

    pub fn init(&mut self) {
        let model = self.controller.model_mut();
        model.add_event_listener(self);

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

impl<'a> EventListener for View<'a> {}
