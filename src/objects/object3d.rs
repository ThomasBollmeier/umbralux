use std::any::Any;
use std::rc::Rc;
use crate::core::{Point, Vector};
use crate::features::material::Material;
use crate::objects::ray::Ray;

pub trait Object3D {
    fn as_any(&self) -> &dyn Any;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, pt: Point) -> Vector;
    fn material(&self) -> Material;
    fn change_material(&self, material: Material);
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

        let over_point = pt + normal * 1E-5;

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
