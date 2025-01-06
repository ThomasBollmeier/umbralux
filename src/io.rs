use std::fs::File;
use std::io::{BufWriter, Write};
use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb, RgbImage};
use num_traits::{cast, zero, FromPrimitive, NumCast, PrimInt};
use crate::core::{Canvas, Color, Number};

pub enum ImageFormat {
    PPM,
    PNG,
    JPG,
}

pub fn save_canvas(file_path: &str, canvas: &Canvas) -> Result<()> {
    let img_format = derive_image_format(file_path)?;
    match img_format {
        ImageFormat::PPM => save_canvas_to_ppm(file_path, canvas),
        ImageFormat::PNG | ImageFormat::JPG => save_canvas_to_file(file_path, canvas),
    }
}

fn derive_image_format(file_path: &str) -> Result<ImageFormat> {
    if file_path.ends_with(".ppm") {
        Ok(ImageFormat::PPM)
    } else if file_path.ends_with(".png") {
        Ok(ImageFormat::PNG)
    } else if file_path.ends_with(".jpg") {
        Ok(ImageFormat::JPG)
    } else {
        Err(anyhow!("Unsupported image format"))
    }
}

fn save_canvas_to_file(file_path: &str, canvas: &Canvas) -> Result<()> {
    let width = canvas.width();
    let height = canvas.height();

    // Create an ImageBuffer to hold the image data
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for row in 0..height {
        for col in 0..width {
            let color = canvas.get_pixel(row, col);
            let scaled_values = color_to_scaled_rgb(&color, 255_u8);
            img.put_pixel(col as u32,
                          row as u32,
                          Rgb([
                              scaled_values[0],
                              scaled_values[1],
                              scaled_values[2]]));
        }
    }

    // Save the image as a PNG file
    img.save(file_path)?;
    Ok(())
}

fn save_canvas_to_ppm(file_path: &str, canvas: &Canvas) -> Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    let content = canvas_to_ppm(canvas);
    writer.write_all(content.as_bytes())?;
    Ok(())
}

fn canvas_to_ppm(canvas: &Canvas) -> String {
    const MAX_COLOR_VALUE: u32 = 255;
    let mut ret = canvas_to_ppm_header(&canvas, MAX_COLOR_VALUE);
    ret.push_str(canvas_to_ppm_data(&canvas, MAX_COLOR_VALUE).as_str());
    ret
}

fn canvas_to_ppm_header(canvas: &Canvas, max_color_value: u32) -> String {
    let mut ret = String::new();
    ret.push_str("P3\n");
    ret.push_str(&format!("{} {}\n", canvas.width(), canvas.height()));
    ret.push_str(&format!("{}\n", {max_color_value}));
    ret
}

fn canvas_to_ppm_data(canvas: &Canvas, max_color_value: u32) -> String {
    const MAX_LINE_SIZE: usize = 70;
    let mut ret = String::new();
    let mut current_line = String::new();

    for row in 0..canvas.height() {
        for col in 0..canvas.width() {
            let color = canvas.get_pixel(row, col);
            let scaled_values = color_to_scaled_rgb(&color, max_color_value);
            for scaled_value in scaled_values {
                let value_str = format!("{scaled_value}");
                if current_line.len() + value_str.len() + 1 <= MAX_LINE_SIZE {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(&value_str);
                } else {
                    ret.push_str(&current_line);
                    ret.push('\n');
                    current_line = value_str;
                }
            }
        }
        if !current_line.is_empty() {
            ret.push_str(&current_line);
            ret.push('\n');
            current_line = String::new();
        }
    }

    if !current_line.is_empty() {
        ret.push_str(&current_line);
        ret.push('\n');
    }

    ret
}

fn color_to_scaled_rgb<T: PrimInt + FromPrimitive + NumCast>(color: &Color, max_color_value: T) -> Vec<T> {
    let color_values: Vec<Number> = Vec::from(color.clone());
    let scaled_values = color_values
        .iter()
        .map(|val| {
            if *val > 1.0 {
                max_color_value
            } else if *val <= 0.0 {
                zero()
            } else {
                let max_color_value: Number = cast(max_color_value)
                    .expect("Cannot convert max color value");
                let scaled = (*val * max_color_value).round();
                T::from_f64(scaled).expect("Could not convert color value")
            }
        })
        .collect::<Vec<T>>();
    scaled_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Canvas, Color};

    #[test]
    fn test_canvas_to_ppm_header(){
        let canvas = Canvas::new(20, 10);
        let max_color_value = 255;

        let expected_ppm_header = "P3\n20 10\n255\n";
        let actual_ppm_header = canvas_to_ppm_header(&canvas, max_color_value);

        assert_eq!(expected_ppm_header, actual_ppm_header);
    }

    #[test]
    fn test_canvas_to_ppm_data(){
        let mut canvas = Canvas::new(5, 3);
        let color1 = Color::new(1.5, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);

        canvas.set_pixel(0, 0, &color1);
        canvas.set_pixel(1, 2, &color2);
        canvas.set_pixel(2, 4, &color3);

        let expected_ppm_data = r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#;
        let actual_ppm_data = canvas_to_ppm_data(&canvas, 255);

        assert_eq!(expected_ppm_data, actual_ppm_data);
    }

    #[test]
    fn test_canvas_to_ppm_data_with_splitting_of_long_lines(){
        let bg_color = Color::new(1.0, 0.8, 0.6);
        let canvas = Canvas::new_with_color(10, 2, &bg_color);

        let expected_ppm_data = r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#;
        let actual_ppm_data = canvas_to_ppm_data(&canvas, 255);

        assert_eq!(expected_ppm_data, actual_ppm_data);
    }

}