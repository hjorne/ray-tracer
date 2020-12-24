pub const EPS: f64 = 0.00001;

pub trait ApproxEq {
    fn xeq(self, other: Self) -> bool;
}

impl ApproxEq for f64 {
    #[inline(always)]
    fn xeq(self, other: Self) -> bool {
        (self - other).abs() < EPS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_tests() {
        assert!((1. + EPS / 2.).xeq(1.));
        assert!(!(1. + EPS * 2.).xeq(1.));
    }
}
