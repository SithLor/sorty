pub mod math;

//pub mod color;

//small
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn core_midpoint() {
        let mid = math::midpoint::get_midpoint(1.0, 1.0, 10.0, 10.0);
        assert!(mid.0 == 1.0 && mid.1 == 10.0);
    }
}
