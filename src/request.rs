use std::error::Error;

use crate::transmission_types::AltStreamResult;

/// Helper function to make a GET request to the AltStream API and return the response body as a string.
pub async fn request(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url).await?.text().await?;

    #[cfg(feature = "api_debug")]
    eprintln!("Response from {}:\n{}", url, body);

    Ok(body)
}

/// Parse a JSON string into a Rust data structure,
/// returning an error if parsing fails.
pub fn parse_response<T>(json_str: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let parsed: T = match serde_json::from_str(json_str) {
        Ok(data) => data,
        Err(err) => {
            return Err(Box::new(std::io::Error::other(format!(
                "Failed to parse JSON: {}\nOriginal JSON: {}",
                err, json_str
            ))));
        }
    };

    Ok(parsed)
}

/// Parse a JSON string into a Rust data structure from a `AltStreamResult` wrapped response,
/// returning an error if parsing fails.
pub fn parse_unwrap_response<T>(json_str: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let wrapped: AltStreamResult<T> = serde_json::from_str(json_str)?;

    wrapped.data()
}
