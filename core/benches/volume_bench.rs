use criterion::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

use minrusty::volume::SpacialMap;

pub fn criterion_benchmark(c: &mut Criterion) {
    // let mut group = c.benchmark_group( ... );
    // group.warm_up_time(Duration::new(1, 0));
    // group.sample_size(1_000);
    // group.measurement_time(Duration::new(2, 0));

    // let mut rng = rand::thread_rng();

    // // TODO

    // group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
