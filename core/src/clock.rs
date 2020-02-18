use std::{thread, time::Duration};

use instant::Instant;

fn system_sleep(duration: Duration) {
    #[cfg(feature = "nat")]
    {
        thread::sleep(duration);
    }

    #[cfg(feature = "web")]
    {
        // TODO: use web-sys to call setTimeout.
        unimplemented!();
    }
}

/// Prevents the application from taking up 100% of the thread.
pub struct Clock {
    start_time: Instant,
    last_time: Instant,
    pub ticks: u64,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            last_time: Instant::now(),
            ticks: 0,
        }
    }

    /// Attempts to sleep about the target time.
    pub fn tick(&mut self, target: Duration) {
        let Self {
            ref mut last_time,
            ref mut ticks,
            ref start_time,
        } = self;

        let now = loop {
            let now = Instant::now();
            let delta_since = now.duration_since(*last_time);
            if delta_since < target {
                let later = target - delta_since;
                system_sleep(later);
            } else {
                break now;
            }
        };

        *last_time = now;
        *ticks += 1;

        if *ticks % 100 == 0 {
            println!(
                "tick {} ({} tps)",
                *ticks,
                *ticks / (now.duration_since(*start_time).as_secs() + 1)
            );
        }
    }
}
