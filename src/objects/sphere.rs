use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use crate::core::{Point, Vector};
use crate::features::material::{Material, MaterialBuilder};
use crate::matrix::Matrix;
use crate::objects::ray::Ray;
use crate::objects::object3d::Object3D;

#[derive(PartialEq, Debug)]
pub struct Sphere {
    origin: Point,
    radius: f64,
    transformation: RefCell<Matrix<f64>>,
    material: RefCell<Material>,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Sphere {
        let transformation = Matrix::identity(4);
        let material = MaterialBuilder::new().build();
        Sphere {
            origin,
            radius,
            transformation: RefCell::new(transformation),
            material: RefCell::new(material) }
    }

    pub fn new_unit() -> Sphere {
        Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0)
    }
}

impl Object3D for Sphere {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let mut ret = Vec::new();

        let a = local_ray.origin();
        let b = local_ray.direction();
        let c = self.origin;
        let d = a - c;

        let b2 = b.dot(b);
        let p = b.dot(d) / b2;
        let q = (d.dot(d) - self.radius * self.radius) / b2;
        let x = p * p - q;

        if x >= 0.0 {
            ret.push(-p - x.sqrt());
            ret.push(-p + x.sqrt());
        }

        ret
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        local_point - self.origin
    }

    fn material(&self) -> Material {
        self.material.borrow().deref().clone()
    }

    fn change_material(&self, material: Material) {
        self.material.replace(material);
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
    use crate::objects::ray::Ray;
    use crate::core::{Vector, Point};
    use crate::objects::object3d::{find_hit, find_intersections, find_many_intersections, Object3D};
    use crate::objects::sphere::Sphere;
    use crate::testutil::{assert_point_eq, assert_vector_eq};
    use crate::transform::{rotation_z, scaling, translation};

    #[test]
    fn ray_intersects_with_sphere_at_two_points() {

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersect(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], 4.0);
        assert_eq!(ts[1], 6.0);
    }

    #[test]
    fn ray_intersects_with_sphere_at_tangent() {

        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersect(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], 5.0);
        assert_eq!(ts[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {

        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersect(&r);

        assert_eq!(ts.len(), 0);
    }

    #[test]
    fn ray_originates_within_sphere() {

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersect(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], -1.0);
        assert_eq!(ts[1], 1.0);
    }

    #[test]
    fn ray_originates_before_sphere() {

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersect(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], -6.0);
        assert_eq!(ts[1], -4.0);
    }

    #[test]
    fn intersection_with_sphere_at_two_points() {

        let rc_r = Rc::new(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)));
        let rc_s: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0));

        let intersections = find_intersections(&rc_r, &rc_s);

        assert_eq!(intersections.len(), 2);
        assert_point_eq(intersections[0].position(), Point::new(0.0, 0.0, -1.0));
        assert_point_eq(intersections[1].position(), Point::new(0.0, 0.0, 1.0));

        let s = rc_s.as_any().downcast_ref::<Sphere>().unwrap();
        let mut s2 = intersections[0].partner_as::<Sphere>();
        assert_eq!(s, s2);
        s2 = intersections[1].partner_as::<Sphere>();
        assert_eq!(s, s2);

    }

    #[test]
    fn hit_for_all_positive_intersections() {

        let rc_r = Rc::new(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)));
        let rc_s1: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0));
        let rc_s2: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 3.0), 1.0));

        let hit = find_hit(find_many_intersections(
            &rc_r,
            &vec![rc_s1.clone(), rc_s2.clone()]));
        assert!(hit.is_some());
        let s1 = rc_s1.as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s1, hit.unwrap().partner_as::<Sphere>());
    }

    #[test]
    fn hit_for_all_negative_intersections() {

        let rc_r = Rc::new(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)));
        let rc_s1: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, -7.0), 1.0));
        let rc_s2: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, -10.0), 1.0));

        let hit = find_hit(find_many_intersections(
            &rc_r, &vec![rc_s1.clone(), rc_s2.clone()]));
        assert!(hit.is_none());
    }

    #[test]
    fn hit_for_some_positive_intersections() {

        let rc_r = Rc::new(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)));
        let rc_s1: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 3.0), 1.0));
        let rc_s2: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, -7.0), 1.0));
        let rc_s3: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0));
        let rc_s4: Rc<dyn Object3D> = Rc::new(Sphere::new(Point::new(0.0, 0.0, -10.0), 1.0));

        let hit = find_hit(find_many_intersections(
            &rc_r,
            &vec![
                rc_s1.clone(),
                rc_s2.clone(),
                rc_s3.clone(),
                rc_s4.clone()]));
        assert!(hit.is_some());
        let s3 = rc_s3.as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s3, hit.unwrap().partner_as::<Sphere>());
    }

    #[test]
    fn intersect_a_scaled_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        sphere.change_transformation(scaling(2.0, 2.0, 2.0));

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_float_absolute_eq!(xs[0], 3.0);
        assert_float_absolute_eq!(xs[1], 7.0);
    }

    #[test]
    fn intersect_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        sphere.change_transformation(translation(5.0, 0.0, 0.0));

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new_unit();
        s.change_transformation(translation(0.0, 1.0, 0.0));
        let pt = Point::new(0.0, 1.0 + std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2);
        let expected = Vector::new(0.0, std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2);
        let actual = s.normal_at(pt);

        assert_vector_eq(actual, expected);
    }


    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new_unit();
        s.change_transformation(scaling(1.0, 0.5, 1.0) * rotation_z(std::f64::consts::PI / 5.0));
        let pt = Point::new(0.0, 0.5 * 2.0_f64.sqrt(), -0.5 * 2.0_f64.sqrt());
        let expected = Vector::new(0.0, 0.9701425, -0.2425356);
        let actual = s.normal_at(pt);

        assert_vector_eq(actual, expected);
    }

}