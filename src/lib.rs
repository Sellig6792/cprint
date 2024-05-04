//! **Cargo-like printing**
//!
//! Easily print beautiful formatted messages like Cargo does.
//!
//! ## Examples
//! ```rust
//! use cprint::{cprint, Color};
//!
//! cprint!("Using cprint crate!");
//! ```
//!
//! ## Coloration
//! For all signatures, you can specify the color of the title with a predefined color from the [`colored::Color`] enum or with RGB values `(r, g, b)`.
//! To specify the color use `=>` at the end of the strings.
//!
//! ## Macros
//! - [`cprint!`] and [`cprintln!`] for printing to stdout.
//! - [`ceprint!`] and [`ceprintln!`] for printing to stderr.
//! - [`cformat!`] for formatting a string.

pub use coloration::{Color, Coloration};

#[doc(hidden)]
pub mod coloration;

#[cfg(feature = "cprint")]
mod cprint;

#[cfg(feature = "ceprint")]
mod ceprint;

#[cfg(feature = "cformat")]
mod cformat;

#[cfg(any(feature = "cprint", feature = "ceprint", feature = "cformat"))]
mod utils;
