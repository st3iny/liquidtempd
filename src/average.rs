pub struct MovingAverage {
    window: usize,
    values: Vec<f32>,
}

impl MovingAverage {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            values: Vec::with_capacity(window),
        }
    }

    pub fn avg(&self) -> Option<f32> {
        if self.values.len() == 0 {
            return None;
        }

        Some(self.values.iter().sum::<f32>() / self.values.len() as f32)
    }

    pub fn push(&mut self, value: f32) {
        self.values.truncate(self.window - 1);
        self.values.insert(0, value);
    }
}
