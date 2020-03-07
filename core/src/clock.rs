use humantime::format_duration;
use instant::Instant;
use specs::prelude::*;
#[cfg(feature = "nat")]
use std::thread;
use std::time::Duration;

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

        use std::convert::TryInto;

        if *ticks % 100 == 0 {
            // println!(
            //     "tick {:6} ({} tps -- {:>16} pt)",
            //     *ticks,
            //     *ticks / (now.duration_since(*start_time).as_secs() + 1),
            //     format_duration(now.duration_since(*start_time) / (*ticks).try_into().unwrap()),
            // );
        }
    }
}

/// Clock system, presumably designed to be thread local.
struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = ();

    fn run(&mut self, _: ()) {}
}
