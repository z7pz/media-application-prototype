pub use crate::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub struct W<T>(pub T); // wrapper


use std::fmt::Display;
pub use std::format as f;
