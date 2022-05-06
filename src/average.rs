pub struct MovingAverage {
    window: usize,
    pointer: usize,
    values: Vec<f32>,
}

impl MovingAverage {
    pub fn new(window: usize) -> Self {
        assert!(window > 0, "Window is zero or negative");
        Self {
            window,
            pointer: 0,
            values: vec![0.0; window],
        }
    }

    pub fn avg(&self) -> f32 {
        self.values.iter().sum::<f32>() / self.window as f32
    }

    pub fn push(&mut self, value: f32) {
        self.values[self.pointer] = value;
        self.pointer = (self.pointer + 1) % self.window;
    }
}
