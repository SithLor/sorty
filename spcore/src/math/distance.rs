macro_rules! if_condition {
    ($count:expr, [$($cond:expr),*], [$($code:block),*]) => {
        {
            let mut result = None;
            $(
                if $cond {
                    result = Some($code);
                }
            )*
            result
        }
    };
}

#[inline(always)]
pub fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let mode = 0;

    if mode == 0 {
        let x = x2 - x1;
        let y = y2 - y1;
        return (x * x + y * y).sqrt();
    } else if mode == 1 {
        return ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    } else if mode == 2 {
        let x = x2 - x1;
        let y = y2 - y1;
        return (x.powf(2.0) + y.powf(2.0)).sqrt();
    } else if mode == 3 {
        return ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    } else if mode == 4 {
        (x2 - x1).hypot(y2 - y1)
    } else {
        return 0.0;
    }
}

