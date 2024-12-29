# Rust Fontawesome Icons

[![Crates.io](https://img.shields.io/crates/v/rust-fontawesome-icons.svg)](https://crates.io/crates/rust-fontawesome-icons)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bircni/rust-fontawesome-icons/blob/main/LICENSE)
[![CI](https://github.com/bircni/rust-fontawesome/actions/workflows/ci.yml/badge.svg)](https://github.com/bircni/rust-fontawesome/actions/workflows/ci.yml)

Get the url to all free fontawesome icons in Rust.

## Usage

```rust
use rust_fontawesome_icons::Icon;

fn main() {
    println!("Hello, world! {}", Icon::RUST.url());
}
```
