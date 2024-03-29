use std::ops::Deref;
use std::rc::Rc;
use crate::core::{Color, Point};
use crate::features::light::{lighting, PointLight};
use crate::objects::object3d::{ComputationResult, find_hit, find_many_intersections, Intersection, Object3D};
use crate::objects::ray::Ray;

pub struct World {
    objects: Vec<Rc<dyn Object3D>>,
    light: Option<Rc<PointLight>>,
}

impl World {

    pub fn new() -> World {
        World {
            objects: vec![],
            light: None,
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
        if let Some(l) = &self.light {
            *l == *light
        } else {
            false
        }
    }

    pub fn set_light(&mut self, light: &Rc<PointLight>) {
        self.light = Some(light.clone());
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

    pub fn get_objects(&self) -> &Vec<Rc<dyn Object3D>> {
        &self.objects
    }

    pub fn shade_hit(&self, comp_res: &ComputationResult) -> Color {
        let is_shadowed = self.is_shadowed(comp_res.over_point);
        lighting(
            &comp_res.object.material(),
            &comp_res.object,
            &self.light.as_ref().unwrap(),
            &comp_res.over_point,
            &comp_res.eye_dir,
            &comp_res.normal,
            is_shadowed
            )
    }

    pub fn color_at_ray_hit(&self, ray: &Rc<Ray>) -> Color {
        let intersections = self.find_intersections(ray);
        match find_hit(intersections) {
            Some(hit) => {
                let comp_res = hit.prepare_computations();
                self.shade_hit(&comp_res)
            }
            None => Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn is_shadowed(&self, pt: Point) -> bool {

        if let Some(light) = &self.light {
            let origin = light.deref().position;
            let direction = pt - origin;
            let ray = Rc::new(Ray::new(origin, direction));
            if let Some(hit) = find_hit(self.find_intersections(&ray)) {
                hit.parameter() < 1.0
            } else {
                false
            }
        } else {
            false // no light => no shadow
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::rc::Rc;
    use crate::core::{Color, Point, Vector};
    use crate::features::light::PointLight;
    use crate::features::material::MaterialBuilder;
    use crate::objects::object3d::{find_hit, find_intersections, Object3D};
    use crate::objects::ray::Ray;
    use crate::objects::sphere::Sphere;
    use crate::objects::world::World;
    use crate::testutil::{assert_color_eq, assert_point_eq, assert_vector_eq};
    use crate::transform::{scaling, translation};

    #[test]
    fn creating_a_world() {

        let world = World::new();

        assert!(world.objects.is_empty());
        assert!(world.light.is_none());
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

    #[test]
    fn shading_an_intersection() {
        let world = create_default_world();
        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, -5.0),
        Vector::new(0.0, 0.0, 1.0)));
        let object = world.get_objects()[0].clone();
        let intersections = find_intersections(&ray, &object);
        let hit = find_hit(intersections).unwrap();
        let comp_res = hit.prepare_computations();
        let expected_color = Color::new(0.3806612, 0.4758265, 0.285496);
        let actual_color = world.shade_hit(&comp_res);

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = create_default_world();
        world.set_light(&Rc::new(PointLight{
            position: Point::new(0.0, 0.25, 0.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        }));
        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0)));
        let object = world.get_objects()[1].clone();
        let intersections = find_intersections(&ray, &object);
        let hit = find_hit(intersections).unwrap();
        let comp_res = hit.prepare_computations();
        let expected_color = Color::new(0.90498447, 0.90498447, 0.90498447);
        let actual_color = world.shade_hit(&comp_res);

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = create_default_world();
        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 1.0, 0.0)));
        let expected_color = Color::new(0.0, 0.0, 0.0);
        let actual_color = world.color_at_ray_hit(&ray);

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = create_default_world();
        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0)));
        let expected_color = Color::new(0.38066119, 0.4758265, 0.285496);
        let actual_color = world.color_at_ray_hit(&ray);

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn the_color_with_an_intersection_behind() {
        let world = create_default_world();
        let objects = world.get_objects();

        let outer_obj = &objects[0];
        let outer_mat = MaterialBuilder::new()
            .color(outer_obj.material().color)
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .shininess(0.0)
            .build();
        outer_obj.change_material(outer_mat);

        let inner_obj = &objects[1];
        let inner_mat = MaterialBuilder::new()
            .color(inner_obj.material().color)
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .shininess(0.0)
            .build();
        inner_obj.change_material(inner_mat.clone());

        let ray = Rc::new(Ray::new(
            Point::new(0.0, 0.0, 0.75),
            Vector::new(0.0, 0.0, -1.0)
        ));

        let actual_color = world.color_at_ray_hit(&ray);
        let expected_color = inner_mat.color;

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        let world = create_default_world();
        let point = Point::new(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let world = create_default_world();
        let point = Point::new(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(point));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let world = create_default_world();
        let point = Point::new(-20.0, -20.0, 20.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let world = create_default_world();
        let point = Point::new(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut world = create_default_world();

        world.set_light(&Rc::new(PointLight{
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Point::new(0.0, 0.0, -10.0),
        }));

        let sphere1: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        world.add_object(&sphere1);

        let sphere2 = Sphere::new_unit();
        sphere2.change_transformation(translation(0.0, 0.0, 10.0));
        let sphere2: Rc<dyn Object3D> = Rc::new(sphere2);
        world.add_object(&sphere2);

        let ray = Rc::new(Ray::new(Point::new(0.0, 0.0, 5.0),
            Vector::new(0.0, 0.0, 1.0)));

        let hit = find_hit(find_intersections(&ray, &sphere2)).unwrap();
        let actual_color = world.shade_hit(&hit.prepare_computations());
        let expected_color = Color::new(0.1, 0.1, 0.1);

        assert_color_eq(expected_color, actual_color);
    }

    #[test]
    fn the_test_should_offset_the_point() {
        let ray = Rc::new(Ray::new(Point::new(0.0,0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0)));

        let sphere = Sphere::new_unit();
        sphere.change_transformation(translation(0.0, 0.0, 1.0));
        let sphere: Rc<dyn Object3D> = Rc::new(sphere);

        let hit = find_hit(find_intersections(&ray, &sphere)).unwrap();
        let comp_res = hit.prepare_computations();

        assert!(comp_res.point.z() > comp_res.over_point.z());
    }

    fn create_light() -> PointLight {
        let light = PointLight{
            intensity: Color::new(1.0, 1.0, 1.0),
            position: Point::new(-10.0, 10.0, -10.0)
        };

        light
    }

    fn create_first_sphere() -> Sphere {
        let sphere = Sphere::new_unit();
        let material = MaterialBuilder::new()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();
        sphere.change_material(material);

        sphere
    }

    fn create_second_sphere() -> Sphere {
        let sphere = Sphere::new_unit();
        sphere.change_transformation(scaling(0.5, 0.5, 0.5));

        sphere
    }

    fn create_world(light: &Rc<PointLight>,
                    sphere_1: &Rc<dyn Object3D>,
                    sphere_2: &Rc<dyn Object3D>) -> World {
        let mut world = World::new();
        world.set_light(light);
        world.add_object(sphere_1);
        world.add_object(sphere_2);

        world
    }

    pub fn create_default_world() -> World {
        let light = Rc::new(create_light());
        let s1: Rc<dyn Object3D> = Rc::new(create_first_sphere());
        let s2: Rc<dyn Object3D> = Rc::new(create_second_sphere());

        create_world(&light, &s1, &s2)
    }

}