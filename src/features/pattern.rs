use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use num_traits::ToPrimitive;
use crate::core::{Color, Point};
use crate::matrix::Matrix;
use crate::objects::object3d::Object3D;
use crate::transform::transform;

pub trait Pattern: Debug {

    fn color_at(&self, pt: Point) -> Color;

    fn color_at_object(&self, object: &Rc<dyn Object3D>, pt: Point) -> Color {
        let object_pt = transform(pt,
                                  &object.transformation().invert().unwrap()).unwrap();
        let pattern_pt = transform(object_pt,
                                  &self.transformation().invert().unwrap()).unwrap();
        self.color_at(pattern_pt)
    }

    fn transformation(&self) -> Matrix<f64>;

    fn change_transformation(&self, transformation: Matrix<f64>);
}

#[derive(Clone, Debug)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
    transformation: RefCell<Matrix<f64>>,
}

impl StripePattern {

    pub fn new(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern {
            color_a,
            color_b,
            transformation: RefCell::new(Matrix::identity(4))
        }
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

impl Pattern for StripePattern {
    fn color_at(&self, pt: Point) -> Color {
        self.stripe_at(pt)
    }

    fn transformation(&self) -> Matrix<f64> {
        self.transformation.borrow().deref().clone()
    }

    fn change_transformation(&self, transformation: Matrix<f64>) {
        self.transformation.replace(transformation);
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::core::{Color, Point};
    use crate::features::pattern::{Pattern, StripePattern};
    use crate::objects::object3d::Object3D;
    use crate::objects::sphere::Sphere;
    use crate::testutil::assert_color_eq;
    use crate::transform::{scaling, translation};

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

    #[test]
    fn stripes_with_an_object_transformation() {
        let (white, _black, stripe_pattern) = initialize();

        let pattern: Rc<dyn Pattern> = Rc::new(stripe_pattern);

        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        object.change_transformation(scaling(2., 2., 2.));

        let pt = Point::new(1.5, 0., 0.);
        let actual_color = pattern.color_at_object(&object, pt);
        assert_color_eq(actual_color, white);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let (white, _black, stripe_pattern) = initialize();

        let pattern: Rc<dyn Pattern> = Rc::new(stripe_pattern);
        pattern.change_transformation(scaling(2., 2., 2.));

        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());

        let pt = Point::new(1.5, 0., 0.);
        let actual_color = pattern.color_at_object(&object, pt);
        assert_color_eq(actual_color, white);
    }

    #[test]
    fn stripes_with_object_and_pattern_transformation() {
        let (white, _black, stripe_pattern) = initialize();

        let pattern: Rc<dyn Pattern> = Rc::new(stripe_pattern);
        pattern.change_transformation(translation(0.5, 0., 0.));

        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        object.change_transformation(scaling(2., 2., 2.));

        let pt = Point::new(2.5, 0., 0.);
        let actual_color = pattern.color_at_object(&object, pt);
        assert_color_eq(actual_color, white);
    }

    fn initialize() -> (Color, Color, StripePattern) {
        let black = Color::new(0.0, 0.0, 0.0);
        let white = Color::new(1.0, 1.0, 1.0);
        (white, black, StripePattern::new(white, black))
    }
}