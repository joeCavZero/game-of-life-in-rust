use std::time::{Instant, Duration};

pub struct Timer {
    ready_time: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            ready_time: Instant::now(), // Inicializa com o momento atual
        }
    }

    pub fn reset(&mut self, time: f32) {
        self.ready_time = Instant::now() + Duration::from_secs_f32(time);
    }

    pub fn is_ready(&self) -> bool {
        Instant::now() >= self.ready_time
    }
}