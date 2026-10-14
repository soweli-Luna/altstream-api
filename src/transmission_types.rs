use std::error::Error;

use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Result-alike enum for AltStream API responses that can either contain data or an error message.
pub(crate) enum AltStreamResult<T> {
    Data(T),
    Error(String),
}

impl<T> AltStreamResult<T> {
    /// Get the data from the result, or return an error if the result is an error.
    ///
    /// Essentially converts the result-alike transmission type into a proper Rust `Result`.
    pub(crate) fn data(self) -> Result<T, Box<dyn Error>> {
        match self {
            AltStreamResult::Data(data) => Ok(data),
            AltStreamResult::Error(err) => Err(Box::new(std::io::Error::other(err))),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// A user's settings.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct UserSettings {
    pub user_number: i64,
    pub name: String,
    pub username: String,
    pub image: String,
    pub header_image: Option<String>,
    pub offline_image: Option<String>,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub bluesky_url: Option<String>,
    pub website_url: Option<String>,
    pub discord_url: Option<String>,
    pub kofi_url: Option<String>,
    pub chat_color: Option<String>,
    pub gaming_shelf: Vec<GameInfo>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601::option")]
    pub followed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub banned_until: Option<OffsetDateTime>,
    pub is_subscriber: bool,
    pub is_alt_stream_supporter: bool,
    pub is_channel_supporter: bool,
    pub is_site_admin: bool,
    pub tester_badge: Option<String>,
    pub channel_points_name: Option<String>,
    pub referral_count: u64,
    pub is_referral_badge_unlocked: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// A channel's emotes.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct ChannelEmotes {
    pub channel: String,
    pub emotes: Vec<ChannelEmoteInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a channel emote.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct ChannelEmoteInfo {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub display_order: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// A channel's rewards.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct ChannelRewards {
    pub channel: String,
    pub rewards: Vec<ChannelRewardInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a channel reward.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct ChannelRewardInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub cost: u64,
    pub enabled: bool,
    pub is_highlight: bool,
    pub cooldown_seconds: Option<u64>,
    pub global_cooldown_seconds: Option<u64>,
    pub max_per_stream: Option<u64>,
    // pub global_max_per_stream: Option<u64>,
    pub max_global_per_stream: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// A channel's panels.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct Panels {
    pub channel: String,
    pub panels: Vec<PanelInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a channel panel.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct PanelInfo {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub content: String, // TODO: this can seemingly be other types as well
    pub display_order: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a channel.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct ChannelInfo {
    pub avatar_url: String,
    pub bio: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    pub display_name: String,
    pub stream: StreamInfo,
    pub username: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a channel's stream.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct StreamInfo {
    pub is_live: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub started_at: Option<OffsetDateTime>,
    pub title: String,
    #[serde(rename = "viewers")]
    pub viewer_count: u64,
    pub game: Option<GameInfo>,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Tags associated with a channel's stream.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct StreamTags {
    pub channel: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "api_debug", serde(deny_unknown_fields))]
/// Information about a game.
///
/// See [AltStream Online API documentation](https://altstream.online/documentation?tab=api).
pub struct GameInfo {
    pub id: i64,
    pub igdb_id: Option<i64>,
    pub name: String,
    pub summary: Option<String>,
    pub slug: String,
    pub cover_url: Option<String>,
    pub ai_details: Option<serde_json::Value>, // TODO: figure out what this actually is
    pub ai_disclosed: Option<bool>,
    pub steam_app_id: Option<String>, // this really should be a number, but the API returns it as a string for some reason
    pub has_steam_disclosure: Option<bool>,
}
