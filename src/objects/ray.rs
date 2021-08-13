use crate::core::{Point, Vector};
use std::any::Any;

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

pub trait Object3D {
    fn as_any(&self) -> &dyn Any;
    fn intersect<'a, 'b>(&'b self, ray: &'a Ray) -> Vec<Intersection<'a, 'b>>;
}

pub struct Intersection<'a, 'b> {
    ray: &'a Ray,
    t: f64,
    partner: &'b dyn Object3D,
}

impl <'a, 'b> Intersection<'a, 'b> {

    pub fn new(ray: &'a Ray, t:f64, partner: &'b dyn Object3D) -> Self {
        Intersection { ray, t, partner }
    }

    pub fn position(&self) -> Point {
        self.ray.position(self.t)
    }

    pub fn partner(&self) -> &dyn Object3D {
        self.partner
    }

}