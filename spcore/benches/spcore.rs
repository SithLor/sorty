

use std::ptr::addr_of_mut;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod _midpoint {
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
    pub fn bench_core_midpoint_wraper(c: &mut Criterion) {
        c.bench_function("core_midpoint_wraper", |b| {
            b.iter(|| {
                let result = spcore::math::midpoint::calculate_midpoint_wraper(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0));
                black_box(result);
            })
        });
        //print the result of the how fast the function is
    }
    pub fn bench_core_midpoint_m(c: &mut Criterion) {
        c.bench_function("core_midpoint_m", |b| {
            b.iter(|| {
                let mut return_1: f64;
                let mut return_2: f64;
                spcore::calculate_midpoint_m!(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0), return_1, return_2);
            })
        });
        //print the result of the how fast the function is
    }
    pub fn bench_core_midpoint_wrapper(c:&mut Criterion){
        c.bench_function("core_midpoint_wrapper", |b| {
            b.iter(|| {
                let result = spcore::math::midpoint::calculate_midpoint_wrapper(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0));
                black_box(result);
            })
        });
    }
    
}
mod _distance {
    use std::ptr::addr_of_mut;

    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    pub fn benchmark_get_distance(c: &mut Criterion) {
        c.bench_function("get_distance", |b| {
            b.iter(|| {
                let result = spcore::math::distance::get_distance_portable(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0));
                black_box(result);
            })
        });
    }
    pub fn benchmark_get_distance_portable(c: &mut Criterion) {
        c.bench_function("get_distance_portable", |b| {
            b.iter(|| {
                let result = spcore::math::distance::get_distance_portable(black_box(1.0), black_box(1.0), black_box(10.0), black_box(10.0));
                black_box(result);
            })
        });
        //print the result of the how fast the function is
    }
}

//criterion_group!(benches, _midpoint::benchmark_get_midpoint,
//                          _midpoint::bench_core_midpoint_static,
//                          _midpoint::bench_core_midpoint_wraper, 
//                          _midpoint::bench_core_midpoint_m,
//                          _distance::benchmark_get_distance,
//                          _distance::benchmark_get_distance_2,
//                          _distance::benchmark_get_distance_3,
//                          _distance::benchmark_get_distance_4
//);
criterion_group!(benches, _distance::benchmark_get_distance_portable);
//criterion_group!(benches, _distance::benchmark_get_distance_static);
criterion_main!(benches);