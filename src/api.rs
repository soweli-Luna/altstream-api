use std::error::Error;

use crate::{request::*, transmission_types::*};

const BASE: &str = "https://altstream.online/api/v1";

/// Get information about a channel.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let channel_info = altstream_api::get_channel_info("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_channel_info(username: &str) -> Result<ChannelInfo, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}")).await?)
}

/// Get information about a channel's panels.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let panels = altstream_api::get_channel_panels("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_channel_panels(username: &str) -> Result<Panels, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/panels")).await?)
}

/// Get information about a channel's rewards.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let rewards = altstream_api::get_channel_rewards("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_channel_rewards(username: &str) -> Result<ChannelRewards, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/rewards")).await?)
}

/// Get information about a channel's emotes.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let emotes = altstream_api::get_channel_emotes("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_channel_emotes(username: &str) -> Result<ChannelEmotes, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/emotes")).await?)
}

/// Get information about a stream's tags.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tags = altstream_api::get_stream_tags("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_stream_tags(username: &str) -> Result<StreamTags, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/tags")).await?)
}

/// Get information about a user's settings.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let settings = altstream_api::get_user_settings("letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_user_settings(username: &str) -> Result<UserSettings, Box<dyn Error>> {
    // user settings is the only endpoint that doesn't return a wrapped response,
    // so we use parse_response instead of parse_unwrap_response
    parse_response(&request(&format!("{BASE}/user-settings/{username}")).await?)
}

/// Get information about a user's settings for a specific channel.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let settings = altstream_api::get_user_channel_settings("soweliluna", "letsgoashe").await?;
/// # Ok(())
/// # }
/// ```
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub async fn get_user_channel_settings(
    username: &str,
    channel: &str,
) -> Result<UserSettings, Box<dyn Error>> {
    parse_response(
        &request(&format!(
            "{BASE}/user-settings/{username}?channel={channel}"
        ))
        .await?,
    )
}
