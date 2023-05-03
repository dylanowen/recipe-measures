use std::fmt::Debug;

use thiserror::Error;

pub use dimension::*;
pub use magnitude::*;
pub use measure::*;
pub use unit::*;

mod dimension;
mod magnitude;
mod measure;
pub mod parser;
mod unit;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown unit: `{0}`")]
    UnknownUnit(String),
    #[error("Found an infinite number when parsing")]
    InfiniteNumber,
}
