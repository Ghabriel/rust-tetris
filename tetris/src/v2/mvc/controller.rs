use super::super::gravity::Gravity;
use super::Model;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Controller {
    model: Rc<RefCell<Model>>,
}

impl Controller {
    pub fn new(model: Rc<RefCell<Model>>) -> Controller {
        Controller { model }
    }

    pub fn change_gravity(&mut self, gravity: Gravity) {
        self.model.borrow_mut().change_gravity(gravity);
    }
}
