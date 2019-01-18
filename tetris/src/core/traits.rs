pub trait Tick {
    fn tick(&mut self, elapsed_time_nanos: u64) -> bool;
}

pub trait Render {
    type Target;

    fn render(&mut self, target: &Self::Target) -> bool;
}
