use std::any::Any;
use crate::core::{Point, Vector};
use crate::features::material::Material;
use crate::matrix::Matrix;
use crate::objects::object3d::Object3D;
use crate::objects::ray::Ray;

pub struct Plane {

}

impl Plane {

    fn new() -> Plane {
        Plane{}
    }

}

impl Object3D for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let dir_y = local_ray.direction().y();
        if dir_y.abs() < f64::EPSILON {
            return vec![]
        }

        vec![-1.0 * local_ray.origin().y() / dir_y]
    }

    fn local_normal_at(&self, local_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn material(&self) -> Material {
        todo!()
    }

    fn change_material(&self, material: Material) {
        todo!()
    }

    fn transformation(&self) -> Matrix<f64> {
        Matrix::identity(4)
    }

    fn change_transformation(&self, transformation: Matrix<f64>) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::core::{Point, Vector};
    use crate::objects::object3d::{find_intersections, Object3D};
    use crate::objects::plane::Plane;
    use crate::objects::ray::Ray;
    use crate::testutil::assert_vector_eq;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::new();
        let n1 = plane.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = plane.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = plane.local_normal_at(Point::new(-5.0, 0.0, 150.0));
        let expected = Vector::new(0.0, 1.0, 0.0);

        assert_vector_eq(expected, n1);
        assert_vector_eq(expected, n2);
        assert_vector_eq(expected, n3);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_plane() {
        let ray = Rc::new(Ray::new(Point::new(0.0, 10.0, 0.0),
            Vector::new(0.0, 0.0, 1.0)));

        test_intersection(&ray, vec![]);
    }

    #[test]
    fn intersect_with_a_ray_coplanar_to_plane() {
        let ray = Rc::new(Ray::new(Point::new(0.0, 0.0, 0.0),
                           Vector::new(0.0, 0.0, 1.0)));

        test_intersection(&ray, vec![]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let ray= Rc::new(Ray::new(Point::new(0.0, 1.0, 0.0),
                       Vector::new(0.0, -1.0, 1.0)));

        test_intersection(&ray, vec![1.0]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let ray= Rc::new(Ray::new(Point::new(0.0, -1.0, 0.0),
                          Vector::new(0.0, 1.0, 1.0)));

        test_intersection(&ray, vec![1.0]);
    }

    fn test_intersection(ray: &Rc<Ray>, expected: Vec<f64>) {
        let plane: Rc<dyn Object3D> = Rc::new(Plane::new());
        let intersections = find_intersections(ray, &plane);

        assert_eq!(intersections.len(), expected.len());

        for (idx, intersection) in intersections.iter().enumerate() {
            assert_float_absolute_eq!(intersection.parameter(), expected[idx]);
        }
    }

}