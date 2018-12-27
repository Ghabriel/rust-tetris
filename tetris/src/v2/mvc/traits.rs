pub trait Tick {
    fn tick(&mut self, elapsed_time: f64) -> bool;
}

pub trait Render {
    type Target;

    fn render(&mut self, target: &Self::Target) -> bool;
}
