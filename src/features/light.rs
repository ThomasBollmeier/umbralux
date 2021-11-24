use crate::core::{Color, Point, Vector};
use crate::features::material::Material;

#[derive(PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    position: &Point,
    camera: &Vector,
    surface: &Vector
) -> Color {

    let normal = surface.normalize();
    let black = Color::new(0.0, 0.0, 0.0);

    // Combine the surface color with the light's color:
    let effective_color = material.color * light.intensity;

    // find direction to light source:
    let light_v = (light.position - *position).normalize();

    let ambient = effective_color * material.ambient;
    let mut diffuse = black;
    let mut specular = black;

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

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use crate::core::{Color, Point, Vector};
    use crate::features::light::{lighting, PointLight};
    use crate::features::material::{Material, MaterialBuilder};
    use crate::testutil::assert_color_eq;

    fn init() -> (Material, Point) {
        let material = MaterialBuilder::new().build();
        let position = Point::new(0.0, 0.0, 0.0);
        (material, position)
    }

    #[test]
    fn lighting_with_camera_between_light_and_surface() {
        let (material, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let expected = Color::new(1.9, 1.9, 1.9);
        let actual = lighting(&material, &light, &position, &camera, &surface);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_between_light_and_surface_offset() {
        let (material, position) = init();
        let camera = Vector::new(0.0, 0.5 * 2.0_f64.sqrt(), -0.5 * 2.0_f64.sqrt());
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let expected = Color::new(1.0, 1.0, 1.0);
        let actual = lighting(&material, &light, &position, &camera, &surface);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_opposite_surface_light_offset_45() {
        let (material, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1 + 0.9 * 0.5 * 2.0_f64.sqrt();
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &light, &position, &camera, &surface);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_camera_in_path_of_reflection() {
        let (material, position) = init();
        let camera = Vector::new(0.0, -0.5*2.0_f64.sqrt(), -0.5*2.0_f64.sqrt());
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1 + 0.9 * 0.5 * 2.0_f64.sqrt() + 0.9;
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &light, &position, &camera, &surface);

        assert_color_eq(expected, actual);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (material, position) = init();
        let camera = Vector::new(0.0, 0.0, -1.0);
        let surface = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight{
            position: Point::new(0.0, 0.0, 10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let intensity = 0.1;
        let expected = Color::new(intensity, intensity, intensity);
        let actual = lighting(&material, &light, &position, &camera, &surface);

        assert_color_eq(expected, actual);
    }

}