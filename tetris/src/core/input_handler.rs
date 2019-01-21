use sfml::window::Key;
use std::collections::HashSet;

pub struct InputHandler {
    valid_pressed_keys: HashSet<Key>,
    previously_pressed_keys: HashSet<Key>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            valid_pressed_keys: HashSet::new(),
            previously_pressed_keys: HashSet::new(),
        }
    }

    pub fn tick(&mut self) {
        let relevant_keys = [
            Key::Left,
            Key::Right,
        ];

        for key in &relevant_keys {
            let is_key_pressed = key.is_pressed();

            if is_key_pressed && !self.previously_pressed_keys.contains(key) {
                self.valid_pressed_keys.insert(*key);
            } else {
                self.valid_pressed_keys.remove(key);
            }

            if is_key_pressed {
                self.previously_pressed_keys.insert(*key);
            } else {
                self.previously_pressed_keys.remove(key);
            }
        }
    }

    pub fn get_pressed_keys(&self) -> impl Iterator<Item = &Key> {
        self.valid_pressed_keys.iter()
    }
}

