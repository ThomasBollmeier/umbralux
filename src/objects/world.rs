use std::rc::Rc;
use crate::features::light::PointLight;
use crate::objects::object3d::{find_many_intersections, Intersection, Object3D};
use crate::objects::ray::Ray;

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

    pub fn find_intersections(&self, ray: &Rc<Ray>) -> Vec<Intersection> {
        let mut intersections = find_many_intersections(ray, &self.objects);
        intersections.sort_by(|i_a, i_b| {
            i_a.parameter().partial_cmp(&i_b.parameter()).unwrap()
        });

        intersections
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::core::{Color, Point, Vector};
    use crate::features::light::PointLight;
    use crate::features::material::MaterialBuilder;
    use crate::objects::object3d::{find_hit, find_intersections, Object3D};
    use crate::objects::ray::Ray;
    use crate::objects::sphere::Sphere;
    use crate::objects::world::World;
    use crate::testutil::{assert_point_eq, assert_vector_eq};
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

    #[test]
    fn intersect_a_world_with_a_ray() {

        let world = create_default_world();
        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0)));

        let intersects = world.find_intersections(&ray);

        assert_eq!(intersects.len(), 4);
        assert_float_absolute_eq!(intersects[0].parameter(), 4.0);
        assert_float_absolute_eq!(intersects[1].parameter(), 4.5);
        assert_float_absolute_eq!(intersects[2].parameter(), 5.5);
        assert_float_absolute_eq!(intersects[3].parameter(), 6.0);

    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {

        let ray = Rc::new(Ray::new(
          Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0,1.0)));
        let shape: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        let hit = find_hit(find_intersections(&ray, &shape)).unwrap();

        let comp_res = hit.prepare_computations();

        assert_eq!(comp_res.t, hit.parameter());
        assert_point_eq(comp_res.point, Point::new(0.0, 0.0, -1.0));
        assert_vector_eq(comp_res.eye_dir, Vector::new(0.0, 0.0, -1.0));
        assert_vector_eq(comp_res.normal, Vector::new(0.0, 0.0, -1.0));
        assert!(!comp_res.inside);
    }

    #[test]
    fn intersection_occurs_on_the_inside() {

        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0,1.0)));
        let shape: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        let hit = find_hit(find_intersections(&ray, &shape)).unwrap();

        let comp_res = hit.prepare_computations();

        assert_eq!(comp_res.t, hit.parameter());
        assert_point_eq(comp_res.point, Point::new(0.0, 0.0, 1.0));
        assert_vector_eq(comp_res.eye_dir, Vector::new(0.0, 0.0, -1.0));
        assert_vector_eq(comp_res.normal, Vector::new(0.0, 0.0, -1.0));
        assert!(comp_res.inside);
    }

    fn create_light() -> PointLight {
        let light = PointLight{
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Point::new(-10.0, 10.0, -10.0)
        };

        light
    }

    fn create_first_sphere() -> Sphere {
        let mut sphere = Sphere::new_unit();
        let material = MaterialBuilder::new()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();
        sphere.set_material(material);

        sphere
    }

    fn create_second_sphere() -> Sphere {
        let mut sphere = Sphere::new_unit();
        sphere.set_transformation(scaling(0.5, 0.5, 0.5));

        sphere
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

    fn create_default_world() -> World {
        let light = Rc::new(create_light());
        let s1: Rc<dyn Object3D> = Rc::new(create_first_sphere());
        let s2: Rc<dyn Object3D> = Rc::new(create_second_sphere());

        create_world(&light, &s1, &s2)
    }

}