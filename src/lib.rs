#[macro_use]
extern crate assert_float_eq;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message}")]
pub struct Error {
    message: String,
}

type Result<T> = std::result::Result<T, crate::Error>;

pub mod core;
pub mod canvas;
pub mod io;
pub mod matrix;
pub mod transform;
pub mod objects;
pub mod features;

pub mod testutil;
