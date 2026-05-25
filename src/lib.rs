//#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod common;
mod error;
pub use common::*;
pub use error::{Error, Result};
pub use zerocopy::{IntoBytes, TryFromBytes};
