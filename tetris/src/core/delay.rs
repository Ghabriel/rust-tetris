pub struct Delay {
    frame_counter: u64,
}

impl Delay {
    pub fn new() -> Delay {
        Delay {
            frame_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frame_counter += 1;
    }

    pub fn block_for_frames(&mut self, num_frames: u64) -> bool {
        if self.frame_counter < num_frames {
            return true;
        }

        self.frame_counter = 0;
        false
    }
}

