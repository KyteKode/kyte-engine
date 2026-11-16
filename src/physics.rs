use std::time::Instant;

pub struct Physics {
    pub tps: u16,
    accumulator: f64,
    previous: Instant
}


impl Physics {
    pub fn init(tps: u16) -> Self {
        Self {
            tps,
            accumulator: 0.0,
            previous: Instant::now()
        }
    }

    pub fn step(&mut self) -> u8 {
        let step_time = 1.0 / (self.tps as f64);

        let now = Instant::now();
        let delta = (now - self.previous).as_secs_f64();
        self.accumulator += delta;

        let mut steps = 0;

        while self.accumulator >= step_time {
            self.accumulator -= step_time;
            steps += 1;
        }

        self.previous = now;

        steps
    }
}