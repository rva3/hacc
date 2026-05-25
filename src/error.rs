use thiserror::Error as TError;

pub use crate::ImageError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(TError, Debug)]
pub enum Error {
    #[error("Image parsing error: {0:?}")]
    Image(#[from] ImageError),

    #[error("Zerocopy error")]
    Zerocopy,
}
