//! > **Cargo-like printing**
//!
//! Easily print beautiful formatted messages like Cargo does.
//!
//! ## Examples
//! ```rust
//! use cprint::{cprint, Color, Coloration};
//!
//! cprint!("Using", "cprint crate!", Color::Green);
//! ```

mod color;
mod cprint_macros;

pub use color::{Color, Coloration};

#[cfg(any(feature = "ceprint"))]
mod ceprint_macros;
