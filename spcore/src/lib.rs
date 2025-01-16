pub mod math;

//pub mod color;

#[no_mangle]
pub extern "C" fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    return (x * x + y * y).sqrt();
}

//small
#[cfg(test)]
mod tests {}
