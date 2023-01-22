#![warn(missing_docs)]
//! > **Cargo-like printing**
//!
//! Easily print beautiful formatted messages like Cargo does.
//!
//! ## Examples
//! ```rust
//! use cprint::{cprint, Color};
//!
//! cprint!("Using", "cprint crate!", Color::Green);
//! ```

mod color;
mod cprint_macros;

pub use color::{Color, Coloration};
