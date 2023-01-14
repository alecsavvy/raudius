use reqwest::Error as ReqError;

/// Error
///
/// All of the errors that this crate can return. Any internal crate errors
/// are usually wrapped in their own variant with the internal type inside
/// in case a consumer of this crate needs the lower level error.
///
/// Request Errors are errors documented in the Audius API and the swagger doc.
/// If a error type exists but is undocumented it will show up under the [Unknown(ReqError)] variant.
#[derive(Debug)]
pub enum Error {
    BuilderError(String),
    ClientError(ReqError),
    HeaderError(String),
    TrackTooLarge(String),
    // Request Errors
    BadRequest(ReqError),
    InternalRedirect(ReqError),
    InternalServerError(ReqError),
    NotFound(ReqError),
    RangeNotSatisfied(ReqError),
    Unknown(ReqError),
}

impl From<ReqError> for Error {
    fn from(e: ReqError) -> Self {
        if let Some(status) = e.status() {
            return match status.as_u16() {
                302 => Self::InternalRedirect(e),
                400 => Self::BadRequest(e),
                404 => Self::NotFound(e),
                416 => Self::RangeNotSatisfied(e),
                500 => Self::InternalServerError(e),
                _ => Self::Unknown(e),
            };
        }
        Self::ClientError(e)
    }
}
