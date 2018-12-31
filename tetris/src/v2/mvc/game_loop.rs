use std::time::{Duration, Instant};
use super::traits::{Render, Tick};

static DEFAULT_UPDATE_FREQUENCY: u8 = 25;

pub struct GameLoop<TUpdate, TRender> {
    update: TUpdate,
    render: TRender,
    running: bool,
    update_period: f64,
}

impl<TUpdate, TRender> GameLoop<TUpdate, TRender>
where
    TUpdate: Tick,
    TRender: Render<Target = TUpdate>,
{
    pub fn new(update: TUpdate, render: TRender) -> GameLoop<TUpdate, TRender> {
        GameLoop {
            update,
            render,
            running: false,
            update_period: 1000.0 / (DEFAULT_UPDATE_FREQUENCY as f64),
        }
    }

    pub fn set_update_frequency(&mut self, ticks_per_second: u8) {
        self.update_period = 1000.0 / (ticks_per_second as f64);
    }

    pub fn start(&mut self) {
        let mut last_measured_time = Instant::now();
        let mut accumulator = 0.0;

        self.running = true;

        while self.running {
            let now = Instant::now();
            let elapsed_time = duration_as_millis(now.duration_since(last_measured_time));
            last_measured_time = now;
            accumulator += elapsed_time as f64;

            while accumulator >= self.update_period {
                if self.update.tick(self.update_period) {
                    self.running = false;
                }

                accumulator -= self.update_period;
            }

            if self.render.render(&self.update) {
                self.running = false;
            }
        }
    }
}

fn duration_as_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + (duration.subsec_millis() as u64)
}
