

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spcore::math::midpoint::get_midpoint;

pub fn benchmark_get_midpoint(c: &mut Criterion) {
    c.bench_function("get_midpoint", |b| {
        b.iter(|| {

            let result = spcore::math::midpoint::get_midpoint(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0));
            black_box(result);
        })
    });
    //print the result of the how fast the function is
}
pub fn bench_core_midpoint_static(c: &mut Criterion) {
    c.bench_function("core_midpoint_static", |b| {
        b.iter(|| {

            static mut X1:f64 = 1.0;
            static mut X2:f64 = 10.0;
            static mut Y1:f64 = 1.0;
            static mut y2:f64 = 10.0;
            static mut return_1: f64 = 0.0;
            static mut return_2: f64 = 0.0;
            unsafe{
                spcore::math::midpoint::calculate_midpoint_static(X1, X2, Y1, y2, &mut return_1, &mut return_2);
            }
        })
    });
    //print the result of the how fast the function is
}
criterion_group!(benches, benchmark_get_midpoint, bench_core_midpoint_static);
criterion_main!(benches);