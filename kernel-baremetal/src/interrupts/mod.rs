#![allow(dead_code)]

pub mod exceptions;
pub mod gic;
pub mod vectors;

pub use gic::init_gic;
pub use vectors::init_exception_vectors;
