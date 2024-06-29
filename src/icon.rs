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
