use crate::core::{Point, Vector};
use std::rc::Rc;

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray{ origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

pub trait Object3D<T> {
    fn intersect(&self, ray: &Rc<Ray>) -> Vec<Intersection<T>>;
}

pub struct Intersection<T> {
    ray: Rc<Ray>,
    t: f64,
    partner: Rc<T>,
}

impl <T> Intersection<T> {

    pub fn new(ray: &Rc<Ray>, t:f64, partner: &Rc<T>) -> Self {
        Intersection {
            ray: Rc::clone(ray),
            t,
            partner: Rc::clone(partner)
        }
    }

    pub fn position(&self) -> Point {
        self.ray.position(self.t)
    }

    pub fn partner(&self) -> &Rc<T> { &self.partner }

}