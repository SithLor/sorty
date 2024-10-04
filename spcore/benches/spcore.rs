

use std::ptr::addr_of_mut;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

            static mut X1:f64 = -98.8;
            static mut X2:f64 = 90.0;
            static mut Y1:f64 = 899.0;
            static mut Y2:f64 = 10.0;
            static mut RETURN_1: f64 = 0.0;
            static mut RETURN_2: f64 = 0.0;

            unsafe {
                spcore::math::midpoint::calculate_midpoint_static(
                    black_box(X1),
                    black_box(X2),
                    black_box(Y1),
                    black_box(Y2),
                    addr_of_mut!(RETURN_1).as_mut().unwrap(),
                    addr_of_mut!(RETURN_2).as_mut().unwrap(),
                );
            }
        })
    });
    //print the result of the how fast the function is
}
criterion_group!(benches, benchmark_get_midpoint, bench_core_midpoint_static);
criterion_main!(benches);