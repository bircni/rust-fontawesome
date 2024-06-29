#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

//! The [Font Awesome][FontAwesome] icon set for Rust.
//!
//! ```
//! use fontawesome::Icon;
//!
//! println!("Hello {}", Icon::Rust);
//! ```
//!
//! [Repository]
//!
//! [FontAwesome]: https://fontawesome.com/
//! [Repository]: https://github.com/vivienm/rust-fontawesome

pub use crate::icon::Icon;
mod icon;
