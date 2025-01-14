//only compile this function if the target supports AVX512
//slow d

//avx256

//test marco for for change impl of

//formal
// r = radius
// h = height
// V = (3.1459 * r^2 * h) / 3
#[inline(always)]
pub fn vol_cone_scalar(radius: f64, height: f64) -> f64 {

    // 1.1
    let mode: u8 = 2;
    match mode {
        1 => (3.1459 * radius.powi(2) * height) / 3.0,
        2 => 1.048633333 * radius * radius * height,
        _ => 0.0,
    }
}

pub fn vol_cone_x86(radius: f64, height: f64) -> f64 {
    return 0.0;
}
