#![allow(dead_code)]
use core::fmt::{self, Display};
use std::borrow::Cow;

include!(concat!(env!("OUT_DIR"), "/icon.rs"));

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

// Implement Into<std::borrow::Cow<'_, str>> for Icon
impl From<Icon> for Cow<'static, str> {
    fn from(icon: Icon) -> Self {
        Cow::from(icon.url())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const FA_VERSION: &str = "6.7.2";
    #[test]
    fn test_display() {
        assert_eq!(
            Icon::RUST.to_string(),
            "https://site-assets.fontawesome.com/releases/v6.7.2/svgs/brands/rust.svg"
        );
    }

    #[test]
    fn test_into_cow() {
        let icon: Cow<'static, str> = Icon::RUST.into();
        assert_eq!(
            icon,
            "https://site-assets.fontawesome.com/releases/v6.7.2/svgs/brands/rust.svg"
        );
    }
}
