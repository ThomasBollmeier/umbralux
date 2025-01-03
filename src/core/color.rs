use std::ops::{Add, Mul, Sub};
use crate::core::{Number, is_number_equal};

#[derive(Debug, Clone)]
pub struct Color {
    red: Number,
    green: Number,
    blue: Number,
}

impl Color {
    pub fn new(red: Number, green: Number, blue: Number) -> Self {
        Self { red, green, blue }
    }

    pub fn red(&self) -> Number {
        self.red
    }

    pub fn green(&self) -> Number {
        self.green
    }

    pub fn blue(&self) -> Number {
        self.blue
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_number_equal(self.red, other.red) &&
            is_number_equal(self.green, other.green) &&
            is_number_equal(self.blue, other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Color::new(1.0, 2.0, 3.0);

        assert!(is_number_equal(c.red(), 1.0));
        assert!(is_number_equal(c.green(), 2.0));
        assert!(is_number_equal(c.blue(), 3.0));
    }

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 + c2;

        assert_eq!(c3, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtract() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 - c2;

        assert_eq!(c3, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_multiply() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 * c2;

        assert_eq!(c3, Color::new(0.63, 0.06, 0.1875));
    }

    #[test]
    fn test_scalar_multiply() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = c * 2.0;

        assert_eq!(c2, Color::new(1.8, 1.2, 1.5));
    }

}
