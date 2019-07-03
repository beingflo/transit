#[derive(Clone)]
pub struct Timer {
    last: f32,
    cur: f32,

    rate: f32,
}

impl Timer {
    pub fn new(rate: f32) -> Self {
        Self { last: 0.0, cur: 0.0, rate }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        self.cur += dt;

        if self.cur - self.last >= self.rate {
            self.last = self.cur;
            true
        } else {
            false
        }
    }
}
