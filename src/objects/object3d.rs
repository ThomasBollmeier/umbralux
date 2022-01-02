use std::any::Any;
use std::rc::Rc;
use crate::core::{Point, Vector};
use crate::features::material::Material;
use crate::matrix::Matrix;
use crate::objects::ray::Ray;
use crate::transform::transform;

pub trait Object3D {

    fn as_any(&self) -> &dyn Any;

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let local_ray = ray.transform(&self.transformation().invert().unwrap());
        self.local_intersect(&local_ray)
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;

    fn normal_at(&self, pt: Point) -> Vector {
        let t_inv = self.transformation().invert().unwrap();
        let local_point = transform(pt,&t_inv).unwrap();
        let local_normal = self.local_normal_at(local_point);
        let t = t_inv.transpose();
        let normal = transform(local_normal, &t).unwrap();

        normal.normalize()
    }

    fn local_normal_at(&self, local_point: Point) -> Vector;

    fn material(&self) -> Material;

    fn change_material(&self, material: Material);

    fn transformation(&self) -> Matrix<f64>;

    fn change_transformation(&self, transformation: Matrix<f64>);
}

pub fn find_intersections(ray: &Rc<Ray>, partner: &Rc<dyn Object3D>) -> Vec<Intersection> {
    partner.intersect(ray).iter().map(|t| {
        Intersection::new(ray, *t, partner)
    }).collect()
}

pub fn find_many_intersections(ray: &Rc<Ray>, partners: &Vec<Rc<dyn Object3D>>) -> Vec<Intersection> {
    let mut ret: Vec<Intersection> = vec![];

    for partner in partners {
        let mut intersections = find_intersections(ray, partner);
        ret.append(&mut intersections);
    }

    ret
}

pub fn find_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut ret: Option<Intersection> = None;

    for intersection in intersections {
        let t = intersection.parameter();
        if t < 0.0 {
            continue;
        }
        ret = match ret {
            Some(intersection_min) => if intersection_min.parameter() <= t {
                Some(intersection_min)
            } else {
                Some(intersection)
            }
            None => Some(intersection)
        }
    }

    ret
}

pub struct Intersection {
    ray: Rc<Ray>,
    t: f64,
    partner: Rc<dyn Object3D>,
}

impl Intersection {

    fn new(ray: &Rc<Ray>, t:f64, partner: &Rc<dyn Object3D>) -> Self {
        Intersection {
            ray: ray.clone(),
            t,
            partner: partner.clone(),
        }
    }

    pub fn parameter(&self) -> f64 {
        self.t
    }

    pub fn position(&self) -> Point {
        self.ray.position(self.t)
    }

    pub fn partner(&self) -> &Rc<dyn Object3D> {
        &self.partner
    }

    pub fn partner_as<T: 'static + Object3D>(&self) -> &T {
        &self.partner.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn prepare_computations(&self) -> ComputationResult {
        let pt = self.ray.position(self.t);
        let eye_dir = -1.0 * self.ray.direction().normalize();
        let mut normal = self.partner.normal_at(pt).normalize();
        let inside = eye_dir.dot(normal) < 0.0;

        if inside {
            normal = -1.0 * normal;
        }

        let over_point = pt + normal * 1.0E-5; // Acne correction

        ComputationResult{
            t: self.t,
            ray: self.ray.clone(),
            object: self.partner.clone(),
            point: pt,
            over_point,
            eye_dir,
            normal,
            inside
        }
    }

}

pub struct ComputationResult {
    pub t: f64,
    pub ray: Rc<Ray>,
    pub object: Rc<dyn Object3D>,
    pub point: Point,
    pub over_point: Point,
    pub eye_dir: Vector,
    pub normal: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};
    use std::rc::Rc;
    use crate::core::{Point, Vector};
    use crate::features::material::MaterialBuilder;
    use crate::matrix::Matrix;
    use crate::objects::object3d::Object3D;
    use crate::objects::sphere::Sphere;
    use crate::testutil::{assert_matrix_float_eq, assert_vector_eq};
    use crate::transform::{rotation_z, scaling, translation};

    #[test]
    fn has_default_transformation() {
        let shape = create_test_shape();

        assert_matrix_float_eq(&shape.transformation(), &Matrix::<f64>::identity(4));
    }

    #[test]
    fn assigning_a_transformation() {
        let shape = create_test_shape();
        let transform = translation(2.0, 3.0, 4.0);
        shape.change_transformation(transform.clone());

        assert_matrix_float_eq(&shape.transformation(), &transform);
    }

    #[test]
    fn has_default_material() {
        let shape = create_test_shape();
        let default_mat = MaterialBuilder::new().build();

        assert_eq!(default_mat, shape.material());
    }

    #[test]
    fn assigning_a_material() {
        let shape = create_test_shape();
        let mat = MaterialBuilder::new()
            .ambient(1.23)
            .build();
        shape.change_material(mat);

        assert_eq!(mat, shape.material());
    }

    #[test]
    fn computing_normal_on_a_translated_shape() {
        let shape = create_test_shape();
        shape.change_transformation(translation(0.0, 1.0, 0.0));
        let expected = Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let pt = Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let actual = shape.normal_at(pt);

        assert_vector_eq(expected, actual);
    }

    #[test]
    fn computing_normal_on_a_transformed_shape() {
        let shape = create_test_shape();
        let t = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        shape.change_transformation(t);
        let expected = Vector::new(0.0, 0.97014, -0.24254);
        let pt = Point::new(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let actual = shape.normal_at(pt);

        assert_vector_eq(expected, actual);
    }


    fn create_test_shape() -> Rc<dyn Object3D> {
        Rc::new(Sphere::new_unit())
    }

}