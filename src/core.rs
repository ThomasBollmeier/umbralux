pub mod base_types;
mod canvas;
mod color;
mod matrix;

pub use base_types::{is_number_equal, Number, Point, Vector, Vec4};
pub use color::Color;
pub use canvas::Canvas;
pub use matrix::Matrix;
