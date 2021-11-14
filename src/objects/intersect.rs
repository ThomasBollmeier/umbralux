use std::any::Any;
use std::rc::Rc;
use crate::core::Point;
use crate::objects::ray::Ray;

pub trait Intersect {
    fn as_any(&self) -> &dyn Any;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}

pub fn find_intersections(ray: &Rc<Ray>,partner: &Rc<dyn Intersect>) -> Vec<Intersection> {
    partner.intersect(ray).iter().map(|t| {
        Intersection::new(ray, *t, partner)
    }).collect()
}

pub struct Intersection {
    ray: Rc<Ray>,
    t: f64,
    partner: Rc<dyn Intersect>,
}

impl Intersection {

    fn new(ray: &Rc<Ray>, t:f64, partner: &Rc<dyn Intersect>) -> Self {
        Intersection {
            ray: ray.clone(),
            t,
            partner: partner.clone(),
        }
    }

    pub fn position(&self) -> Point {
        self.ray.position(self.t)
    }

    pub fn partner(&self) -> &Rc<dyn Intersect> {
        &self.partner
    }

}
