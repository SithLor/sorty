use std::arch::asm;

struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
}
//Point-slope form
//y − y_1 = m(x − x_1)

// ...existing code...
#[inline(always)]
pub fn point_slope_form(y: f64, x: f64, m: f64, x1: f64, y1: f64) -> f64 {
    let result: f64;
    unsafe {
        asm!(
            // Calculate (x - x1)
            "movsd xmm0, {x}",
            "subsd xmm0, {x1}",
            // Calculate (y - y1)
            "movsd xmm1, {y}",
            "subsd xmm1, {y1}",
            // Perform m * (x - x1) + (y - y1)
            "vfmadd231sd xmm0, {m}, xmm1",
            // Move the result to the output variable
            "movsd {result}, xmm0",
            x = in(reg) x,
            x1 = in(reg) x1,
            m = in(reg) m,
            y = in(reg) y,
            y1 = in(reg) y1,
            result = out(reg) result,
            options(nostack, preserves_flags)
        );
    }
    result
}
// ...existing code...

pub mod distance;
pub mod midpoint;
pub mod volume;
pub const ONE_HALF: f64 = 0.5;
pub const ONE_QAURTER: f64 = 0.25;
pub const THREE_OUT_OF_FOUR: f64 = 0.75;
pub const ONE_HOLE: f64 = 1.0;
