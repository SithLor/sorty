#[no_mangle]
pub fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    (x * x + y * y).sqrt()
}
#[no_mangle]
pub fn get_distance_optimized(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    let x2 = x * x;
    let y2 = y * y;
    (x2 + y2).sqrt()
}
fn main() {

}
