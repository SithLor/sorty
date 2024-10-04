use crate::math::ONE_HALF;
/// Computes the midpoint of two points `(x1, y1)` and `(x2, y2)`.
///
/// # Arguments
///
/// * `x1` - The x-coordinate of the first point.
/// * `x2` - The x-coordinate of the second point.
/// * `y1` - The y-coordinate of the first point.
/// * `y2` - The y-coordinate of the second point.
///
/// # Returns
///
/// A tuple `(f64, f64)` representing the midpoint of the two points.
///
/// # Examples
/// 
/// ```
/// use spcore::math::midpoint::get_midpoint;
/// let (x1, y1) = (1.0,1.0);
/// let (x2, y2) = (10.0,10.0);
/// let midpoint = get_midpoint(x1, x2, y1, y2);
/// ```
/// Performance: 100.58026563501256 nanoseconds
#[inline(always)]
pub fn get_midpoint(x1: f64, x2: f64, y1: f64, y2: f64) -> (f64, f64) {
    ((x1 + x2) * ONE_HALF, (y1 + y2) * ONE_HALF)
}

pub fn calculate_midpoint_static(x1: f64, x2: f64, y1: f64, y2: f64, return_1: &mut f64, return_2: &mut f64) {
    *return_1 = (x1 + x2) * ONE_HALF;
    *return_2 = (y1 + y2) * ONE_HALF;
}

//macro_rules! calculate_midpoint_wraper {
//    ($x1:expr, $x2:expr, $y1:expr, $y2:expr, $return_1:expr, $return_2:expr) => {
//        $return_1 = ($x1 + $x2) * ONE_HALF;
//        $return_2 = ($y1 + $y2) * ONE_HALF;
//    };
//}
macro_rules! calculate_midpoint_wraper {
    ($x1:expr, $x2:expr, $y1:expr, $y2:expr, $return_1:expr, $return_2:expr) => {
        static mut X1:f64 = $x1;
        static mut X2:f64 = $x2;
        static mut Y1:f64 = $y1;
        static mut Y2:f64 = $y2;
        static mut RETURN_1: f64 = 0.0;
        static mut RETURN_2: f64 = 0.0;
        
    };
}

//calculate_midpoint_asm
//