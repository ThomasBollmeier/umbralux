use std::rc::Rc;
use crate::features::light::PointLight;
use crate::objects::object3d::Object3D;

pub struct World {
    objects: Vec<Rc<dyn Object3D>>,
    lights: Vec<PointLight>,
}

impl World {
    
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![]
        }
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: &Rc<dyn Object3D>) {
        self.objects.push(object.clone());
    }

}

#[cfg(test)]
mod tests {

}