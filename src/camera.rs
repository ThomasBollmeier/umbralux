use std::rc::Rc;
use crate::canvas::Canvas;
use crate::core::Point;
use crate::matrix::Matrix;
use crate::objects::ray::Ray;
use crate::objects::world::World;
use crate::transform::transform;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix<f64>,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {

    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let (pixel_size, half_width, half_height) =
            Camera::init_sizes(field_of_view, hsize, vsize);

        Camera{
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::<f64>::identity(4),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    pub fn set_transformation(&mut self, t: Matrix<f64>) {
        self.transform = t;
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    fn init_sizes(field_of_view: f64, hsize: usize, vsize: usize) -> (f64, f64, f64) {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_width / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        (half_width * 2.0 / (hsize as f64), half_width, half_height)
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates in world space:
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let t = self.transform.invert().unwrap();

        let pixel = transform(Point::new(world_x, world_y, -1.0), &t)
            .unwrap();
        let origin = transform(Point::new(0.0, 0.0, 0.0), &t)
            .unwrap();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut ret = Canvas::new(self.hsize, self.vsize);
        let mut ray: Rc<Ray>;

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                ray = Rc::new(self.ray_for_pixel(x, y));
                ret.set_pixel(x, y, world.color_at_ray_hit(&ray));
            }
        }

        ret
    }

}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::core::{Color, Point, Vector};
    use crate::matrix::Matrix;
    use crate::objects::ray::Ray;
    use crate::objects::world::tests;
    use crate::testutil::{assert_color_eq, assert_matrix_float_eq, assert_point_eq, assert_vector_eq};
    use crate::transform::{rotation_y, translation, view_transform};

    #[test]
    fn constructing_a_camera() {
        let hsize: usize = 160;
        let vsize: usize = 120;
        let field_of_view = std::f64::consts::FRAC_PI_2;

        let camera = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(hsize, camera.hsize());
        assert_eq!(vsize, camera.vsize());
        assert_float_absolute_eq!(field_of_view, camera.field_of_view());
        assert_matrix_float_eq(&Matrix::<f64>::identity(4), camera.transform());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let camera = Camera::new(
            200,
            125,
            std::f64::consts::FRAC_PI_2);

        assert_float_absolute_eq!(0.01, camera.pixel_size());
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let camera = Camera::new(
            125,
            200,
            std::f64::consts::FRAC_PI_2);

        assert_float_absolute_eq!(0.01, camera.pixel_size());
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let camera = Camera::new(
            201,
            101,
            std::f64::consts::FRAC_PI_2);
        let ray: Ray = camera.ray_for_pixel(100, 50);

        assert_point_eq(Point::new(0.0, 0.0, 0.0),
                        ray.origin());
        assert_vector_eq(Vector::new(0.0, 0.0, -1.0),
                         ray.direction());
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let camera = Camera::new(
            201,
            101,
            std::f64::consts::FRAC_PI_2);
        let ray: Ray = camera.ray_for_pixel(0, 0);

        assert_point_eq(Point::new(0.0, 0.0, 0.0),
                        ray.origin());
        assert_vector_eq(Vector::new(0.66519, 0.33259, -0.66851),
                         ray.direction());
    }

    #[test]
    fn ray_through_center_of_canvas_with_transformed_camera() {
        let mut camera = Camera::new(
            201,
            101,
            std::f64::consts::FRAC_PI_2);
        camera.set_transformation(rotation_y(std::f64::consts::FRAC_PI_4) *
            translation(0.0, -2.0, 5.0));
        let ray: Ray = camera.ray_for_pixel(100, 50);

        assert_point_eq(Point::new(0.0, 2.0, -5.0),
                        ray.origin());
        assert_vector_eq(Vector::new(2.0_f64.sqrt() * 0.5, 0.0, -2.0_f64.sqrt() * 0.5),
                         ray.direction());
    }

    #[test]
    fn rendering_a_world_with_a_camera() {

        let world = tests::create_default_world();

        let mut camera = Camera::new(11, 11, std::f64::consts::FRAC_PI_2);

        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        camera.set_transformation(view_transform(from, to, up));

        let image = camera.render(&world);

        let expected = Color::new(0.38066, 0.47583, 0.2855);
        let actual = image.get_pixel(5, 5);

        assert_color_eq(expected, actual);




    }
}