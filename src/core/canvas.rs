use crate::core::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let black = Color::new(0.0, 0.0, 0.0);
        Self::new_with_color(width, height, &black)
    }

    pub fn new_with_color(width: usize, height: usize, color: &Color) -> Canvas {
        let pixels = vec![vec![color.clone(); width]; height];
        Canvas {width, height, pixels}
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> Color {
        self.pixels[row][col].clone()
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: &Color) {
        self.pixels[row][col] = pixel.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::core::canvas::Canvas;
    use crate::core::Color;

    #[test]
    fn can_create_canvas() {
        let canvas = Canvas::new(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);

        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 20);

        for row in 0..canvas.height() {
            for col in 0..canvas.width() {
                assert_eq!(canvas.get_pixel(row, col), black);
            }
        }
    }

    #[test]
    fn can_create_canvas_with_bg_color() {
        let bg_color = Color::new(0.0, 1.0, 0.0);
        let canvas = Canvas::new_with_color(5, 5, &bg_color);

        for row in 0..canvas.height() {
            for col in 0..canvas.width() {
                assert_eq!(canvas.get_pixel(row, col), bg_color);
            }
        }
    }

    #[test]
    fn can_set_pixel() {
        let bg_color = Color::new(0.0, 0.0, 0.0);
        let red = Color::new(1.0, 0.0, 0.0);
        let mut canvas = Canvas::new_with_color(10, 20, &bg_color);
        canvas.set_pixel(0, 0, &red);
        assert_eq!(canvas.get_pixel(0, 0), red);
    }

}