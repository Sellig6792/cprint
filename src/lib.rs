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

pub use coloration::{Color, Coloration};

pub mod coloration;
#[cfg(feature = "cprint")]
mod cprint;

#[cfg(feature = "ceprint")]
mod ceprint;

#[cfg(feature = "cformat")]
mod cformat;

#[cfg(any(feature = "cprint", feature = "ceprint", feature = "cformat"))]
mod utils;
