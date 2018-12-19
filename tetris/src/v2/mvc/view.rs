use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use super::super::gravity::Gravity;
use super::{EventListener, MC};
use std::cell::RefCell;
use std::ops::DerefMut;

pub struct View<'a> {
    mvc: RefCell<MC<'a>>,
    window: RenderWindow,
}

impl<'a> View<'a> {
    pub fn new<'b>(
        mvc: RefCell<MC<'b>>,
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
            mvc,
            window,
        }
    }

    pub fn init(&mut self) {
        let window = &mut self.window;
        let shape = CircleShape::new(100., 30);

        let mut mvc = self.mvc.borrow_mut();
        let MC { ref mut controller, ref mut model } = mvc.deref_mut();
        controller.change_gravity(model, Gravity::Naive);

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
