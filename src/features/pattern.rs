use num_traits::ToPrimitive;
use crate::core::{Color, Point};

pub struct StripePattern {
    color_a: Color,
    color_b: Color,
}

impl StripePattern {

    pub fn new(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern { color_a, color_b }
    }

    pub fn stripe_at(&self, pt: Point) -> Color {
        let x = pt.x();
        if x.floor().to_i64().unwrap() % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    pub fn color_a(&self) -> Color {
        self.color_a
    }

    pub fn color_b(&self) -> Color {
        self.color_b
    }

}


#[cfg(test)]
mod tests {
    use crate::core::{Color, Point};
    use crate::features::pattern::StripePattern;
    use crate::testutil::assert_color_eq;

    #[test]
    fn creating_a_stripe_pattern() {
        let (white, black, pattern) = initialize();

        assert_color_eq(pattern.color_a(), white);
        assert_color_eq(pattern.color_b(), black);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let (white, _black, pattern) = initialize();

        assert_color_eq(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let (white, _black, pattern) = initialize();

        assert_color_eq(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let (white, black, pattern) = initialize();

        assert_color_eq(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), white);
        assert_color_eq(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), black);
        assert_color_eq(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), black);
        assert_color_eq(pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)), black);
        assert_color_eq(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), white);
    }

    fn initialize() -> (Color, Color, StripePattern) {
        let black = Color::new(0.0, 0.0, 0.0);
        let white = Color::new(1.0, 1.0, 1.0);
        (white, black, StripePattern::new(white, black))
    }
}