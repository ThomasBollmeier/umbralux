use std::rc::Rc;
use crate::core::{Color, Point, Vector};
use crate::features::material::Material;
use crate::objects::object3d::Object3D;

#[derive(PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

pub fn lighting(
    material: &Material,
    object: &Rc<dyn Object3D>,
    light: &PointLight,
    position: &Point,
    camera: &Vector,
    surface: &Vector,
    in_shadow: bool
) -> Color {

    let normal = surface.normalize();
    let black = Color::new(0.0, 0.0, 0.0);

    // Determine color to work with:
    let color = if let Some(pattern) = &material.pattern {
        pattern.color_at_object(object, *position)
    } else {
        material.color
    };

    // Combine the surface color with the light's color:
    let effective_color = color * light.intensity;

    // find direction to light source:
    let light_v = (light.position - *position).normalize();

    let ambient = effective_color * material.ambient;
    let mut diffuse = black;
    let mut specular = black;

    if !in_shadow {
        // light_dot_normal reperesents the cosine of the angle between the light vector
        // and the normal vector. A negative number means the light is on the outer side of
        // the surface.
        let light_dot_normal = light_v.dot(normal);

        if light_dot_normal >= 0.0 {
            // compute the diffuse contribution
            diffuse = effective_color * material.diffuse * light_dot_normal;

            // reflect_dot_camera represents the cosine of the angle between the reflection
            // vector and the camera vector. A negative number means the light reflects
            // away from the camera.
            let reflect_v = -1.0 * light_v.reflect(&normal);
            let reflect_dot_camera = reflect_v.dot(*camera);

            if reflect_dot_camera > 0.0 {
                let factor = reflect_dot_camera.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::core::{Color, Point, Vector};
    use crate::features::light::{lighting, PointLight};
    use crate::features::material::{Material, MaterialBuilder};
    use crate::features::pattern::{Pattern, StripePattern};
    use crate::objects::object3d::Object3D;
    use crate::objects::sphere::Sphere;
    use crate::testutil::assert_color_eq;

    fn init() -> (Material, Rc<dyn Object3D>, Point) {
        let material = MaterialBuilder::new().build();
        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());
        let position = Point::new(0.0, 0.0, 0.0);
        (material, object, position)
    }

    #[test]
    fn lighting_with_camera_between_light_and_surface() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let expected = Color::new(1.9, 1.9, 1.9);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, false);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_between_light_and_surface_offset() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, 0.5 * 2.0_f64.sqrt(), -0.5 * 2.0_f64.sqrt());
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let expected = Color::new(1.0, 1.0, 1.0);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, false);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_opposite_surface_light_offset_45() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1 + 0.9 * 0.5 * 2.0_f64.sqrt();
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, false);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_in_path_of_reflection() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, -0.5*2.0_f64.sqrt(), -0.5*2.0_f64.sqrt());
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1 + 0.9 * 0.5 * 2.0_f64.sqrt() + 0.9;
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, false);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, 10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1;
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, false);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (material, object, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1;
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &object, &light, &position, &camera, &surface, true);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let pattern: Rc<dyn Pattern> = Rc::new(StripePattern::new(
            Color::new(1.0, 1.0, 1.0),
            Color::new(0.0, 0.0, 0.0)
        ));
        let material = MaterialBuilder::new()
            .pattern(&pattern)
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .build();
        let object: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let pt1 = Point::new(0.9, 0.0, 0.0);
        let pt2 = Point::new(1.1, 0.0, 0.0);
        let expected1 = Color::new(1.0, 1.0, 1.0);
        let expected2 = Color::new(0.0, 0.0, 0.0);

        assert_color_eq(expected1,
                        lighting(&material, &object, &light, &pt1,
                                 &eyev, &normalv, false));
        assert_color_eq(expected2,
                        lighting(&material, &object, &light, &pt2,
                                 &eyev, &normalv, false));

    }

}