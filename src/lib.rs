#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

//! The Font Awesome icon url set for Rust.
//!
//! ```
//! use rust_fontawesome_icons::Icon;
//!
//! println!("URL to the rust logo: {}", Icon::RUST.url());
//! ```
//!
//! [Repository]
//!
//! [FontAwesome]: https://fontawesome.com/
//! [Repository]: https://github.com/bircni/rust-fontawesome

pub use crate::icon::Icon;
mod icon;
