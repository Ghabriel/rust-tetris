use super::super::gravity::Gravity;
use super::Model;

pub struct Controller {}

impl Controller {
    pub fn new() -> Controller {
        Controller { }
    }

    pub fn change_gravity(&mut self, model: &mut Model, gravity: Gravity) {
        model.change_gravity(gravity);
    }
}
