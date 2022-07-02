//! Batteries-included CLI output library for Rust.

#[doc(hidden)]
pub mod global;

mod config;
mod macros;
mod xmt;

pub use crate::xmt::XMT;
pub use colored::Color;
pub use config::{Config, Level, OutputMode, Style};
pub use global::{init, init_default, nest, pick};
