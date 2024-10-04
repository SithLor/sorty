pub fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    (x * x + y * y).sqrt()
}
