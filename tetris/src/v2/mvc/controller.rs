use super::super::gravity::Gravity;
use super::Model;

pub struct Controller<'a> {
    model: &'a mut Model,
}

impl<'a> Controller<'a> {
    pub fn new(model: &mut Model) -> Controller {
        Controller { model }
    }

    pub fn change_gravity(&mut self, gravity: Gravity) {
        self.model.change_gravity(gravity);
    }
}
