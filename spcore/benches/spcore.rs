use std::ptr::addr_of_mut;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod speed_test {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use spcore::get_distance;
    pub fn benchmark_get_volume(c: &mut Criterion) {
        c.bench_function("vol_cone_scalar", |b| {
            b.iter(|| {
                let result = get_distance(
                    black_box(1.0),
                    black_box(1.0),
                    black_box(1.0),
                    black_box(1.0),
                );
                black_box(result);
            })
        });
    }
}

criterion_group!(benches, speed_test::benchmark_get_volume);
//criterion_group!(benches, _distance::benchmark_get_distance_static);
criterion_main!(benches);
