pub use crate::base::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Wrapper tuple newtype struct
pub struct NewT<T>(pub T);

// Maybe other shortcuts here
pub use std::format as fmt;
