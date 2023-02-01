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

#[cfg(feature = "coloration")]
pub use coloration::{Color, Coloration};

#[cfg(feature = "coloration")]
pub mod coloration;

#[cfg(feature = "cprint")]
mod cprint;

#[cfg(feature = "ceprint")]
mod ceprint;
