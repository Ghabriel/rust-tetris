use super::super::settings::Settings;

pub trait BoardGravityPair {
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings);
}
