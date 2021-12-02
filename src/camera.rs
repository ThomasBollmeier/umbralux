use crate::matrix::Matrix;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix<f64>,
}

impl Camera {

    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        Camera{
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::<f64>::identity(4)
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

    pub fn pixel_size(&self) -> f64 {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;
        let half_width: f64;

        if aspect >= 1.0 {
            half_width = half_view;
        } else {
            half_width = half_view * aspect;
        }

        half_width * 2.0 / (self.hsize as f64)
    }

}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::matrix::Matrix;
    use crate::testutil::assert_matrix_float_eq;

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


}