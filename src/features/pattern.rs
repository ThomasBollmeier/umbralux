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
enum PatternKind {
    Stripes,
    Gradient,
    Ring,
    Checkers3D,
}

#[derive(Clone, Debug)]
pub struct TwoColorPattern {
    kind: PatternKind,
    color_a: Color,
    color_b: Color,
    transformation: RefCell<Matrix<f64>>,
}

impl TwoColorPattern {

    pub fn new_stripes(color_a: Color, color_b: Color) -> TwoColorPattern {
        Self::new(PatternKind::Stripes, color_a, color_b)
    }

    pub fn new_gradient(color_a: Color, color_b: Color) -> TwoColorPattern {
        Self::new(PatternKind::Gradient, color_a, color_b)
    }

    pub fn new_ring(color_a: Color, color_b: Color) -> TwoColorPattern {
        Self::new(PatternKind::Ring, color_a, color_b)
    }

    pub fn new_checkers3d(color_a: Color, color_b: Color) -> TwoColorPattern {
        Self::new(PatternKind::Checkers3D, color_a, color_b)
    }

    fn new(kind: PatternKind, color_a: Color, color_b: Color) -> TwoColorPattern {
        TwoColorPattern {
            kind,
            color_a,
            color_b,
            transformation: RefCell::new(Matrix::identity(4))
        }
    }

    fn stripes_color_at(&self, pt: Point) -> Color {
        let x = pt.x();
        if x.floor().to_i64().unwrap() % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn gradient_color_at(&self, pt: Point) -> Color {

        let gradient = self.color_b - self.color_a;
        let fraction = pt.x() - pt.x().floor();

        self.color_a + gradient * fraction
    }

    fn ring_color_at(&self, pt: Point) -> Color {

        let radius = (pt.x().powi(2) + pt.z().powi(2)).sqrt()
            .floor().to_i64().unwrap();

        if radius % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn checkers3d_color_at(&self, pt: Point) -> Color {
        let value = pt.x().floor() + pt.y().floor() + pt.z().floor();
        let value = value as i64;

        if value % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

}

impl Pattern for TwoColorPattern {

    fn color_at(&self, pt: Point) -> Color {
        match self.kind {
            PatternKind::Stripes => self.stripes_color_at(pt),
            PatternKind::Gradient => self.gradient_color_at(pt),
            PatternKind::Ring => self.ring_color_at(pt),
            PatternKind::Checkers3D => self.checkers3d_color_at(pt),
        }
    }

    fn transformation(&self) -> Matrix<f64> {
        self.transformation.borrow().deref().clone()
    }

    fn change_transformation(&self, transformation: Matrix<f64>) {
        self.transformation.replace(transformation);
    }
}

#[derive(Clone, Debug)]
pub struct NestedPattern {
    kind: PatternKind,
    pattern_a: Rc<dyn Pattern>,
    pattern_b: Rc<dyn Pattern>,
    transformation: RefCell<Matrix<f64>>,
}

impl NestedPattern {

    pub fn new_stripes(pattern_a: Rc<dyn Pattern>, pattern_b: Rc<dyn Pattern>) -> NestedPattern {
        Self::new(PatternKind::Stripes, pattern_a, pattern_b)
    }

    pub fn new_gradient(pattern_a: Rc<dyn Pattern>, pattern_b: Rc<dyn Pattern>) -> NestedPattern {
        Self::new(PatternKind::Gradient, pattern_a, pattern_b)
    }

    pub fn new_ring(pattern_a: Rc<dyn Pattern>, pattern_b: Rc<dyn Pattern>) -> NestedPattern {
        Self::new(PatternKind::Ring, pattern_a, pattern_b)
    }

    pub fn new_checkers3d(pattern_a: Rc<dyn Pattern>, pattern_b: Rc<dyn Pattern>) -> NestedPattern {
        Self::new(PatternKind::Checkers3D, pattern_a, pattern_b)
    }

    fn new(kind: PatternKind, pattern_a: Rc<dyn Pattern>, pattern_b: Rc<dyn Pattern>) -> NestedPattern {
        NestedPattern {
            kind,
            pattern_a: pattern_a.clone(),
            pattern_b: pattern_b.clone(),
            transformation: RefCell::new(Matrix::identity(4))
        }
    }

    fn stripes_color_at(&self, pt: Point) -> Color {
        let x = pt.x();
        if x.floor().to_i64().unwrap() % 2 == 0 {
            self.pattern_a.color_at(pt)
        } else {
            self.pattern_b.color_at(pt)
        }
    }

    fn gradient_color_at(&self, pt: Point) -> Color {

        let gradient = self.pattern_b.color_at(pt) - self.pattern_a.color_at(pt);
        let fraction = pt.x() - pt.x().floor();

        self.pattern_a.color_at(pt) + gradient * fraction
    }

    fn ring_color_at(&self, pt: Point) -> Color {

        let radius = (pt.x().powi(2) + pt.z().powi(2)).sqrt()
            .floor().to_i64().unwrap();

        if radius % 2 == 0 {
            self.pattern_a.color_at(pt)
        } else {
            self.pattern_b.color_at(pt)
        }
    }

    fn checkers3d_color_at(&self, pt: Point) -> Color {
        let value = pt.x().floor() + pt.y().floor() + pt.z().floor();
        let value = value as i64;

        if value % 2 == 0 {
            self.pattern_a.color_at(pt)
        } else {
            self.pattern_b.color_at(pt)
        }
    }

}

impl Pattern for NestedPattern {

    fn color_at(&self, pt: Point) -> Color {
        match self.kind {
            PatternKind::Stripes => self.stripes_color_at(pt),
            PatternKind::Gradient => self.gradient_color_at(pt),
            PatternKind::Ring => self.ring_color_at(pt),
            PatternKind::Checkers3D => self.checkers3d_color_at(pt),
        }
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
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use crate::core::{Color, Point};
    use crate::features::pattern::{Pattern, TwoColorPattern};
    use crate::matrix::Matrix;
    use crate::objects::object3d::Object3D;
    use crate::objects::sphere::Sphere;
    use crate::testutil::{assert_color_eq, assert_matrix_float_eq};
    use crate::transform::{scaling, translation};

    #[derive(Clone, Debug)]
    pub struct TestPattern {
        transformation: RefCell<Matrix<f64>>,
    }

    impl TestPattern {
        fn new() -> TestPattern {
            TestPattern {
                transformation: RefCell::new(Matrix::<f64>::identity(4))
            }
        }
    }

    impl Pattern for TestPattern {

        fn color_at(&self, pt: Point) -> Color {
            Color::new(pt.x(), pt.y(), pt.z())
        }

        fn transformation(&self) -> Matrix<f64> {
            self.transformation.borrow().deref().clone()
        }

        fn change_transformation(&self, transformation: Matrix<f64>) {
            self.transformation.replace(transformation);
        }
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let (white, _black, pattern) = initialize();

        assert_color_eq(pattern.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.color_at(Point::new(0.0, 1.0, 0.0)), white);
        assert_color_eq(pattern.color_at(Point::new(0.0, 2.0, 0.0)), white);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let (white, _black, pattern) = initialize();

        assert_color_eq(pattern.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.color_at(Point::new(0.0, 0.0, 1.0)), white);
        assert_color_eq(pattern.color_at(Point::new(0.0, 0.0, 2.0)), white);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let (white, black, pattern) = initialize();

        assert_color_eq(pattern.color_at(Point::new(0.0, 0.0, 0.0)), white);
        assert_color_eq(pattern.color_at(Point::new(0.9, 0.0, 0.0)), white);
        assert_color_eq(pattern.color_at(Point::new(1.0, 0.0, 0.0)), black);
        assert_color_eq(pattern.color_at(Point::new(-0.1, 0.0, 0.0)), black);
        assert_color_eq(pattern.color_at(Point::new(-1.0, 0.0, 0.0)), black);
        assert_color_eq(pattern.color_at(Point::new(-1.1, 0.0, 0.0)), white);
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

    #[test]
    fn assigning_a_transformation() {
        let pattern = TestPattern::new();
        let expected = translation(1., 2., 3.);
        pattern.change_transformation(expected.clone());

        let actual = pattern.transformation();
        assert_matrix_float_eq(&actual, &expected);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let pattern = TestPattern::new();
        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        object.change_transformation(scaling(2., 2., 2.));
        let expected = Color::new(1., 1.5, 2.);
        let actual = pattern.color_at_object(&object, Point::new(2., 3., 4.));

        assert_color_eq(actual, expected);
    }

    #[test]
    fn a_pattern_with_an_pattern_transformation() {
        let pattern = TestPattern::new();
        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        pattern.change_transformation(scaling(2., 2., 2.));
        let expected = Color::new(1., 1.5, 2.);
        let actual = pattern.color_at_object(&object, Point::new(2., 3., 4.));

        assert_color_eq(actual, expected);
    }

    #[test]
    fn a_pattern_with_both_transformations() {
        let pattern = TestPattern::new();
        pattern.change_transformation(translation(0.5, 1., 1.5));
        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        object.change_transformation(scaling(2., 2., 2.));
        let expected = Color::new(0.75, 0.5, 0.25);
        let actual = pattern.color_at_object(&object, Point::new(2.5, 3., 3.5));

        assert_color_eq(actual, expected);
    }

    #[test]
    fn gradient_linearly_interpolates() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = TwoColorPattern::new_gradient(white, black);

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );

        assert_color_eq(
            pattern.color_at(Point::new(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );

        assert_color_eq(
            pattern.color_at(Point::new(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );

    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = TwoColorPattern::new_ring(white, black);

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(1., 0., 0.)),
            black
        );

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 1.)),
            black
        );

        assert_color_eq(
            pattern.color_at(Point::new(0.708, 0., 0.708)),
            black
        );


    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = TwoColorPattern::new_checkers3d(white, black);

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0.99, 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(1.01, 0., 0.)),
            black
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = TwoColorPattern::new_checkers3d(white, black);

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0., 0.99, 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0., 1.01, 0.)),
            black
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let black = Color::new(0., 0., 0.);
        let white = Color::new(1., 1., 1.);
        let pattern = TwoColorPattern::new_checkers3d(white, black);

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 0.99)),
            white
        );

        assert_color_eq(
            pattern.color_at(Point::new(0., 0., 1.01)),
            black
        );
    }


    fn initialize() -> (Color, Color, TwoColorPattern) {
        let black = Color::new(0.0, 0.0, 0.0);
        let white = Color::new(1.0, 1.0, 1.0);
        (white, black, TwoColorPattern::new_stripes(white, black))
    }
}