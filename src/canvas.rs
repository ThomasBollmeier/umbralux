use crate::core::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {

    pub fn new(width: usize, height: usize) -> Canvas {
        Self::with_background(width, height, Color::new(0.0, 0.0, 0.0))
    }

    pub fn with_background(width: usize, height: usize, bg_color: Color) -> Canvas {
        let mut pixels = Vec::new();

        for _x in 0..width {
            let mut column = Vec::new();
            for _y in 0..height {
                column.push(bg_color);
            }
            pixels.push(column);
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn get_dimension(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[x][y] = c;
    }
}

// ============================================================================

#[cfg(test)]
mod tests {

    use super::Canvas;
    use super::super::core::Color;

    #[test]
    fn create_canvas() {

        let width = 200;
        let height = 100;
        let canvas = Canvas::new(width, height);

        assert_canvas(canvas, width, height, Color::new(0.0,0.0,0.0));
    }

    #[test]
    fn create_canvas_with_background() {

        let width = 200;
        let height = 100;
        let red = Color::new(1.0,0.0, 0.0);
        let canvas = Canvas::with_background(width, height, red);

        assert_canvas(canvas, width, height, red);
    }

    #[test]
    fn set_pixel() {

        let width = 200;
        let height = 100;
        let x = 100;
        let y = 50;
        let black = Color::new(0.0,0.0, 0.0);
        let white = Color::new(1.0,1.0, 1.0);
        let mut canvas = Canvas::new(width, height);

        canvas.set_pixel(x, y, white);

        let mut pixel: Color;

        for yy in 0..height {
            for xx in 0..width {
                pixel = canvas.get_pixel(xx, yy);
                if xx == x && yy == y {
                    assert_eq!(pixel, white);
                } else {
                    assert_eq!(pixel, black);
                }
            }
        }

    }

    fn assert_canvas(canvas: Canvas, width: usize, height: usize, bg_color: Color) {

        let (w, h) = canvas.get_dimension();
        assert_eq!(width, w);
        assert_eq!(height, h);

        let mut pixel: Color;
        for y in 0..h {
            for x in 0..w {
                pixel = canvas.get_pixel(x, y);
                assert_eq!(pixel, bg_color);
            }
        }

    }

}