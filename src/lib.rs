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
pub use color::{Color, Coloration};

#[cfg(feature = "cprint")]
mod cprint_macros;

#[cfg(feature = "ceprint")]
mod ceprint_macros;
