use std::any::Any;
use crate::core::Point;
use crate::objects::ray::Ray;
use crate::objects::intersect::Intersect;

#[derive(PartialEq, Debug)]
pub struct Sphere {
    origin: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Sphere {
        Sphere { origin, radius }
    }

    pub fn intersects_with_ray_at(&self, ray: &Ray) -> Vec<f64> {
        let mut ret = Vec::new();
        let a = ray.origin();
        let b = ray.direction();
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
}

impl Intersect for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        self.intersects_with_ray_at(ray)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::objects::ray::Ray;
    use crate::core::{Vector, Point};
    use crate::objects::intersect::{find_intersections, Intersect};
    use crate::objects::sphere::Sphere;
    use crate::testutil::assert_point_eq;

    #[test]
    fn ray_intersects_with_sphere_at_two_points() {

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersects_with_ray_at(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], 4.0);
        assert_eq!(ts[1], 6.0);
    }

    #[test]
    fn ray_intersects_with_sphere_at_tangent() {

        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersects_with_ray_at(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], 5.0);
        assert_eq!(ts[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {

        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersects_with_ray_at(&r);

        assert_eq!(ts.len(), 0);
    }

    #[test]
    fn ray_originates_within_sphere() {

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersects_with_ray_at(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], -1.0);
        assert_eq!(ts[1], 1.0);
    }

    #[test]
    fn ray_originates_before_sphere() {

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let ts = s.intersects_with_ray_at(&r);

        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], -6.0);
        assert_eq!(ts[1], -4.0);
    }

    #[test]
    fn intersection_with_sphere_at_two_points() {

        let rc_r = Rc::new(Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)));
        let rc_s: Rc<dyn Intersect> = Rc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0));

        let intersections = find_intersections(&rc_r, &rc_s);

        assert_eq!(intersections.len(), 2);
        assert_point_eq(intersections[0].position(), Point::new(0.0, 0.0, -1.0));
        assert_point_eq(intersections[1].position(), Point::new(0.0, 0.0, 1.0));

        let s = rc_s.as_any().downcast_ref::<Sphere>().unwrap();
        let mut s2 = intersections[0].partner().as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s, s2);
        s2 = intersections[1].partner().as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s, s2);

    }

}