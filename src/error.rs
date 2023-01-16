use thiserror::Error;

#[derive(Error, Debug)]
pub enum CurlError {
    #[error("header error")]
    InvalidHeaderValue,

    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("from_utf8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
