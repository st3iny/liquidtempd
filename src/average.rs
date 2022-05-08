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
        if self.values.len() < self.window {
            self.values.push(value);
        } else {
            self.values[self.pointer] = value;
            self.pointer = (self.pointer + 1) % self.window;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let mut ma = MovingAverage::new(2);
        ma.push(1.0);
        ma.push(5.0);
        assert_eq!(ma.avg(), Some(3.0));
    }

    #[test]
    fn test_window_overflow() {
        let mut ma = MovingAverage::new(2);
        ma.push(1.0);
        ma.push(5.0);
        ma.push(7.0);
        assert_eq!(ma.avg(), Some(6.0));
    }

    #[test]
    fn test_partial_window() {
        let mut ma = MovingAverage::new(3);
        ma.push(1.0);
        assert_eq!(ma.avg(), Some(1.0));
        ma.push(5.0);
        assert_eq!(ma.avg(), Some(3.0));
    }

    #[test]
    fn test_empty() {
        let ma = MovingAverage::new(3);
        assert_eq!(ma.avg(), None);
    }
}
