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
        todo!()
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        todo!()
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
        todo!()
    }

    fn change_transformation(&self, transformation: Matrix<f64>) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::core::{Point, Vector};
    use crate::objects::object3d::Object3D;
    use crate::objects::plane::Plane;
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

}