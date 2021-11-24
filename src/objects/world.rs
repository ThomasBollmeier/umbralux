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

    pub fn contains_object<T: 'static + PartialEq + Object3D>(&self, an_object: &Rc<dyn Object3D>) -> bool  {
        let an_object = an_object.as_any().downcast_ref::<T>().unwrap();
        for object in &self.objects {
            if let Some(t) = object.as_any().downcast_ref::<T>() {
                if an_object == t {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_light(&self, light: &Rc<PointLight>) -> bool  {
        self.lights.contains(light)
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
    use std::rc::Rc;
    use crate::core::{Color, Point};
    use crate::features::light::PointLight;
    use crate::features::material::MaterialBuilder;
    use crate::objects::object3d::Object3D;
    use crate::objects::sphere::Sphere;
    use crate::objects::world::World;
    use crate::transform::scaling;

    #[test]
    fn creating_a_world() {

        let world = World::new();

        assert!(world.objects.is_empty());
        assert!(world.lights.is_empty());

    }

    #[test]
    fn default_world() {

        let light = Rc::new(create_light());
        let s1: Rc<dyn Object3D> = Rc::new(create_first_sphere());
        let s2: Rc<dyn Object3D> = Rc::new(create_second_sphere());

        let world = create_world(&light, &s1, &s2);

        assert!(world.contains_light(&light));
        assert!(world.contains_object::<Sphere>(&s1));
        assert!(world.contains_object::<Sphere>(&s2));
    }

    fn create_light() -> PointLight {
        let light = PointLight{
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Point::new(-10.0, 10.0, -10.0)
        };

        light
    }

    fn create_first_sphere() -> Sphere {
        let mut s = Sphere::new_unit();
        let material = MaterialBuilder::new()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();
        s.set_material(material);

        s
    }

    fn create_second_sphere() -> Sphere {
        let mut s = Sphere::new_unit();
        s.set_transformation(scaling(0.5, 0.5, 0.5));

        s
    }

    fn create_world(light: &Rc<PointLight>,
                    sphere_1: &Rc<dyn Object3D>,
                    sphere_2: &Rc<dyn Object3D>) -> World {
        let mut world = World::new();
        world.add_light(light);
        world.add_object(sphere_1);
        world.add_object(sphere_2);

        world
    }

}