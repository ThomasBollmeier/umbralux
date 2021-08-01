use crate::core::Point;
use crate::objects::ray::Ray;

pub struct Sphere {
    origin: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Sphere {
        Sphere{ origin, radius}
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

#[cfg(test)]
mod tests {
    use crate::objects::ray::Ray;
    use crate::core::{Vector, Point};
    use crate::objects::sphere::Sphere;

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
}