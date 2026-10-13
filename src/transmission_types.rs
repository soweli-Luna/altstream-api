
use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Result-alike enum for AltStream API responses that can either contain data or an error message.
pub enum AltStreamResult<T> {
    Data(T),
    Error(String),
}

impl<T> AltStreamResult<T> {
    /// Get the data from the result, or return an error if the result is an error.
    ///
    /// Essentially converts the result-alike transmission type into a proper Rust `Result`.
    pub fn data(self) -> Result<T, Box<dyn Error>> {
        match self {
            AltStreamResult::Data(data) => Ok(data),
            AltStreamResult::Error(err) => Err(Box::new(std::io::Error::other(err))),
        }
    }
}
