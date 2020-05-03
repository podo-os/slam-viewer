use std::time;

pub struct Timer {
    time: time::Instant,
    time_limit: time::Duration,
}

impl Timer {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_new(time_limit: time::Duration) -> Option<Self> {
        Some(Self {
            time: std::time::Instant::now(),
            time_limit,
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn try_new(_time_limit: time::Duration) -> Option<Self> {
        None
    }

    pub fn sync(&mut self) {
        let elapsed = self.time.elapsed();

        if elapsed < self.time_limit {
            let remain = self.time_limit - elapsed;

            std::thread::sleep(remain);
        }

        self.time = time::Instant::now();
    }
}
