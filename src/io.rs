use crate::canvas::Canvas;
use crate::core::Color;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn export_as_ppm(canvas: &Canvas, file_name: &str) -> Result<()> {
    let mut ppm_file = File::create(Path::new(file_name))?;
    let max_color_value = 255;

    write!(
        &mut ppm_file,
        "{}",
        create_ppm_header(max_color_value, canvas)
    )?;

    Ok(())
}

fn create_ppm_header(max_color_value: i32, canvas: &Canvas) -> String {
    let mut ret = String::new();
    let (width, height) = canvas.get_dimension();

    ret.push_str("P3\n");
    ret.push_str(&format!("{} {}\n", width, height));
    ret.push_str(&format!("{}\n", max_color_value));
    ret
}

fn create_ppm_body(max_color_value: i32, canvas: &Canvas) -> String {
    let mut ret = String::new();
    let max_line_size = 70;
    let (width, height) = canvas.get_dimension();

    for y in 0..height {
        let mut line = String::new();

        for x in 0..width {
            let pixel = canvas.get_pixel(x, y);

            let red = color_value_to_ppm_str(pixel.red(), max_color_value);
            line = append(&mut ret, &mut line, max_line_size, &red);

            let green = color_value_to_ppm_str(pixel.green(), max_color_value);
            line = append(&mut ret, &mut line, max_line_size, &green);

            let blue = color_value_to_ppm_str(pixel.blue(), max_color_value);
            line = append(&mut ret, &mut line, max_line_size, &blue);
        }

        if !line.is_empty() {
            ret.push_str(&format!("{}\n", line));
        }
    }

    ret
}

fn append(
    body: &mut String,
    curr_line: &mut String,
    max_line_size: usize,
    new_str: &str,
) -> String {
    let new_len = if !curr_line.is_empty() {
        curr_line.len() + 1 + new_str.len()
    } else {
        new_str.len()
    };

    if new_len <= max_line_size {
        if !curr_line.is_empty() {
            curr_line.push_str(" ");
        }
        curr_line.push_str(new_str);
        String::from(&curr_line[..])
    } else {
        body.push_str(&format!("{}\n", curr_line));
        String::from(new_str)
    }
}

fn color_value_to_ppm_str(color: f64, max_color_value: i32) -> String {
    if color >= 0.0 {
        let mut scaled = (color * max_color_value as f64).round() as i32;
        if scaled > max_color_value {
            scaled = max_color_value;
        }
        format!("{}", scaled)
    } else {
        "0".to_string()
    }
}

// ============================================================================

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::core::Color;
    use crate::io::{create_ppm_body, create_ppm_header};

    #[test]
    fn header_creation() {
        let canvas = Canvas::new(5, 3);
        let header = create_ppm_header(255, &canvas);

        assert_eq!(
            header,
            "P3\n\
                    5 3\n\
                    255\n"
        );
    }

    #[test]
    fn body_creation() {
        let mut canvas = Canvas::new(5, 3);
        canvas.set_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.set_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.set_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        let body = create_ppm_body(255, &canvas);

        assert_eq!(
            body,
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        );
    }

    #[test]
    fn body_creation_w_long_lines() {
        let bg_color = Color::new(1.0, 0.8, 0.6);
        let canvas = Canvas::with_background(10, 2, bg_color);

        let body = create_ppm_body(255, &canvas);

        assert_eq!(
            body,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                          153 255 204 153 255 204 153 255 204 153 255 204 153\n\
                          255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                          153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        );
    }
}
