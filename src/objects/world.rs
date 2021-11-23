use std::rc::Rc;
use crate::features::light::PointLight;
use crate::objects::object3d::Object3D;

pub struct World {
    objects: Vec<Rc<dyn Object3D>>,
    lights: Vec<Rc<PointLight>>,
}

impl World {

    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_light(&mut self, light: &Rc<PointLight>) {
        self.lights.push(light.clone());
    }

    pub fn add_object(&mut self, object: &Rc<dyn Object3D>) {
        self.objects.push(object.clone());
    }

}

#[cfg(test)]
mod tests {
    use crate::objects::world::World;

    #[test]
    fn creating_a_world() {

        let world = World::new();

        assert!(world.objects.is_empty());
        assert!(world.lights.is_empty());

    }

}