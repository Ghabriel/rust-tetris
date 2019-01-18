use std::time::{Duration, Instant};
use super::super::helpers::FrequencyGauge;
use super::traits::{Render, Tick};

const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

static DEFAULT_UPDATE_FREQUENCY: u8 = 25;

pub struct GameLoop<TUpdate, TRender> {
    update: TUpdate,
    render: TRender,
    running: bool,
    update_period: u64,
    frequency_gauge: FrequencyGauge,
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
            update_period: NANOSECONDS_PER_SECOND / (DEFAULT_UPDATE_FREQUENCY as u64),
            frequency_gauge: FrequencyGauge::new(),
        }
    }

    pub fn set_update_frequency(&mut self, ticks_per_second: u8) {
        self.update_period = NANOSECONDS_PER_SECOND / (ticks_per_second as u64);
    }

    pub fn start(&mut self) {
        let mut last_measured_time = Instant::now();
        let mut accumulator = 0;

        self.running = true;
        self.frequency_gauge.reset();

        while self.running {
            let now = Instant::now();
            let elapsed_time = duration_as_nanos(now.duration_since(last_measured_time));
            last_measured_time = now;
            accumulator += elapsed_time;

            while accumulator >= self.update_period {
                self.frequency_gauge.tick();

                if self.frequency_gauge.get_tick_count() % 25 == 0 {
                    println!("Tick rate: {}", self.frequency_gauge.measure());
                }

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

fn duration_as_nanos(duration: Duration) -> u64 {
    duration.as_secs() * NANOSECONDS_PER_SECOND + (duration.subsec_nanos() as u64)
}
