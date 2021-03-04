use crate::impl_eq;
use derive_more::{Add, Constructor, Div, Mul, Neg, Sub};

use super::float::*;
use std::string::ToString;

#[derive(Debug, Copy, Clone, Default, Add, Sub, Mul, Div, Neg, Constructor)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl_eq!(Color; red, green, blue);

impl std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        fn f64_to_i64(value: f64) -> i64 {
            let vi64 = (value * 255.0) as i64;

            if vi64 > 255 {
                255
            } else if vi64 < 0 {
                0
            } else {
                vi64
            }
        }
        format!(
            "{} {} {}",
            f64_to_i64(self.red),
            f64_to_i64(self.green),
            f64_to_i64(self.blue)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_constructor_test() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_add_test() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_sub_test() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_mult_scalar_test() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_mult_other_test() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
