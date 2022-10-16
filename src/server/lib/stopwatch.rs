use std::time::Instant;

pub struct Stopwatch {
    start: Instant,
}

#[allow(dead_code)]
impl Stopwatch {
    pub fn new() -> Self {
        return Stopwatch {
            start: Instant::now(),
        };
    }

    pub fn stop(&self) -> u128 {
        return self.start.elapsed().as_millis();
    }

    pub fn restart(&mut self) -> () {
        return self.start = Instant::now();
    }
}
