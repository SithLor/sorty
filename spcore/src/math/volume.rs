//only compile this function if the target supports AVX512

#[inline(always)]
pub unsafe fn vol_cone_avx512(radius: f64, height: f64) -> f64 {
    use std::arch::x86_64::*;
    let radius_vec = _mm512_set1_pd(radius);
    let height_vec = _mm512_set1_pd(height);
    let pi_vec = _mm512_set1_pd(3.14159);
    let three_vec = _mm512_set1_pd(3.0);

    let r_sq = _mm512_mul_pd(radius_vec, radius_vec);
    let vol = _mm512_mul_pd(_mm512_mul_pd(pi_vec, r_sq), height_vec);
    let vol = _mm512_div_pd(vol, three_vec);

    let mut result = [0.0_f64; 8];
    _mm512_storeu_pd(result.as_mut_ptr(), vol);
    result[0]
}
#[inline(always)]
pub fn vol_cone_scalar(radius: f64, height: f64) -> f64 {
    (3.1459 * radius.powi(2) * height) / 3.0
}
