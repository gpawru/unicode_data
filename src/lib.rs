#[macro_use]
extern crate lazy_static;

pub mod codepoint;
mod normalization;
mod parse;

pub use normalization::*;
pub use parse::*;
