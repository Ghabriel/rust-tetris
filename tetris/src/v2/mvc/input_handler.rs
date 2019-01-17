use sfml::window::Key;
use std::collections::HashSet;

pub struct InputHandler {
    pressed_keys: HashSet<Key>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn tick(&mut self) {
        let relevant_keys = [
            Key::Left,
            Key::Right,
        ];

        for key in &relevant_keys {
            let is_key_pressed = key.is_pressed();

            if is_key_pressed && !self.pressed_keys.contains(key) {
                self.pressed_keys.insert(*key);
            } else if !is_key_pressed {
                self.pressed_keys.remove(key);
            }
        }
    }

    pub fn get_pressed_keys(&self) -> impl Iterator<Item = &Key> {
        self.pressed_keys.iter()
    }
}

