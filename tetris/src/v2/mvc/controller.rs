use super::super::gravity::Gravity;
use super::Model;
use std::rc::Rc;

pub struct Controller {
    model: Rc<Model>,
}

impl Controller {
    pub fn new(model: Rc<Model>) -> Controller {
        Controller { model }
    }

    pub fn change_gravity(&mut self, gravity: Gravity) {
        Rc::get_mut(&mut self.model).unwrap().change_gravity(gravity);
    }
}
