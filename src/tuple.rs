use super::float::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const ZERO: Tuple = Tuple {x: 0., y: 0., z: 0., w: 0.}; 

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Default for Tuple {
    fn default() -> Self {
        ZERO
    }
}

impl Clone for Tuple {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Tuple {}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.xeq(other.x) && self.y.xeq(other.y) && self.z.xeq(other.z) && self.w.xeq(other.w)
    }
}

impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, c: f64) -> Self {
        Tuple::new(c * self.x, c * self.y, c * self.z, c * self.w)
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        Tuple::new(
            self * tuple.x,
            self * tuple.y,
            self * tuple.z,
            self * tuple.w,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, c: f64) -> Self {
        Tuple::new(self.x / c, self.y / c, self.z / c, self.w / c)
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn pnt(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let l = self.norm();
        Tuple::new(self.x / l, self.y / l, self.z / l, self.w / l)
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        if !self.w.xeq(0.) || !other.w.xeq(0.) {
            panic!("Cross product on 4D tuple");
        }

        Tuple::vec(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_constructor_test() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
    }

    #[test]
    fn tuple_pnt_test() {
        let a = Tuple::pnt(4.3, -4.2, 3.1);
        assert_eq!(a, Tuple::new(4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn tuple_vec_test() {
        let a = Tuple::vec(4.3, -4.2, 3.1);
        assert_eq!(a, Tuple::new(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn tuple_eq_test() {
        let a = Tuple::pnt(0.0, 1.0, 2.0);
        let b = Tuple::pnt(0.0 + EPS / 4.0, 1.0 - EPS / 4.0, 2.0 + EPS / 4.0);
        assert_eq!(a, b);

        let a = Tuple::pnt(0.0, 1.0, 2.0);
        let b = Tuple::pnt(0.0 + EPS * 4.0, 1.0, 2.0);
        assert_ne!(a, b);
    }

    #[test]
    fn tuple_add_test() {
        let a1 = Tuple::pnt(3., -2., 5.);
        let a2 = Tuple::vec(-2., 3., 1.);

        assert_eq!(a1 + a2, Tuple::pnt(1., 1., 6.));
    }

    #[test]
    fn tuple_sub_test() {
        let p1 = Tuple::pnt(3., 2., 1.);
        let p2 = Tuple::pnt(5., 6., 7.);
        assert_eq!(p1 - p2, Tuple::vec(-2., -4., -6.));

        let p = Tuple::pnt(3., 2., 1.);
        let v = Tuple::vec(5., 6., 7.);
        assert_eq!(p - v, Tuple::pnt(-2., -4., -6.));

        let v1 = Tuple::vec(3., 2., 1.);
        let v2 = Tuple::vec(5., 6., 7.);
        assert_eq!(v1 - v2, Tuple::vec(-2., -4., -6.));

        let v = Tuple::vec(1., -2., 3.);
        assert_eq!(ZERO - v, Tuple::vec(-1., 2., -3.));
    }

    #[test]
    fn tuple_neg_test() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn tuple_scalar_mult_test() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
        assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn tuple_scalar_div_test() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a / 2., Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn tuple_norm_test() {
        assert!(Tuple::vec(1., 0., 0.).norm().xeq(1.));
        assert!(Tuple::vec(0., 1., 0.).norm().xeq(1.));
        assert!(Tuple::vec(0., 0., 1.).norm().xeq(1.));
        assert!(Tuple::vec(1., 2., 3.).norm().xeq(14f64.sqrt()));
        assert!(Tuple::vec(-1., -2., -3.).norm().xeq(14f64.sqrt()));
    }

    #[test]
    fn tuple_normalize_test() {
        assert_eq!(Tuple::vec(4., 0., 0.).normalize(), Tuple::vec(1., 0., 0.));
        assert_eq!(
            Tuple::vec(1., 2., 3.).normalize(),
            Tuple::vec(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        );

        assert!(Tuple::vec(1., 2., 3.).normalize().norm().xeq(1.));
    }

    #[test]
    fn tuple_dot_test() {
        let a = Tuple::vec(1., 2., 3.);
        let b = Tuple::vec(2., 3., 4.);
        assert!(a.dot(&b).xeq(20.));
    }

    #[test]
    fn tuple_cross_test() {
        let a = Tuple::vec(1., 2., 3.);
        let b = Tuple::vec(2., 3., 4.);

        assert_eq!(a.cross(&b), Tuple::vec(-1., 2., -1.));
        assert_eq!(b.cross(&a), Tuple::vec(1., -2., 1.));
    }
}
