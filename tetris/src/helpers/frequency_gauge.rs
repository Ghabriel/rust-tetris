use std::time::{Duration, Instant};

pub struct FrequencyGauge {
    counter: usize,
    start_time: Instant,
}

impl FrequencyGauge {
    pub fn new() -> FrequencyGauge {
        FrequencyGauge {
            counter: 0,
            start_time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.start_time = Instant::now();
    }

    pub fn tick(&mut self) {
        self.counter += 1;
    }

    pub fn get_tick_count(&self) -> usize {
        self.counter
    }

    pub fn measure(&self) -> f64 {
        let now = Instant::now();
        let tick_count = duration_as_millis(now.duration_since(self.start_time));

        (self.counter as f64) * 1000. / (tick_count as f64)
    }
}

fn duration_as_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + (duration.subsec_millis() as u64)
}
