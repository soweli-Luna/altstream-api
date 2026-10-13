use std::{error::Error, format};

use serde::Deserialize;
use time::OffsetDateTime;

const BASE: &str = "https://altstream.online/api/v1";

/// Helper function to make a GET request to the AltStream API and return the response body as a string.
async fn request(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url).await?.text().await?;

    #[cfg(debug_assertions)]
    eprintln!("Response from {}:\n{}", url, body);

    Ok(body)
}

/// Parse a JSON string into a Rust data structure,
/// returning an error if parsing fails.
fn parse_response<T>(json_str: &str) -> Result<T, Box<dyn Error>>
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
fn parse_unwrap_response<T>(json_str: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let wrapped: AltStreamResult<T> = serde_json::from_str(json_str)?;

    wrapped.data()
}

pub async fn get_channel_info(username: &str) -> Result<ChannelInfo, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}")).await?)
}

pub async fn get_channel_panels(username: &str) -> Result<Panels, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/panels")).await?)
}

pub async fn get_channel_rewards(username: &str) -> Result<ChannelRewards, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/rewards")).await?)
}

pub async fn get_channel_emotes(username: &str) -> Result<ChannelEmotes, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/emotes")).await?)
}

pub async fn get_stream_tags(username: &str) -> Result<StreamTags, Box<dyn Error>> {
    parse_unwrap_response(&request(&format!("{BASE}/channels/{username}/tags")).await?)
}

pub async fn get_user_settings(username: &str) -> Result<UserSettings, Box<dyn Error>> {
    // user settings is the only endpoint that doesn't return a wrapped response,
    // so we use parse_response instead of parse_unwrap_response
    parse_response(&request(&format!("{BASE}/user-settings/{username}")).await?)
}

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
/// Result-alike enum for AltStream API responses that can either contain data or an error message.
enum AltStreamResult<T> {
    Data(T),
    Error(String),
}

impl<T> AltStreamResult<T> {
    fn data(self) -> Result<T, Box<dyn Error>> {
        match self {
            AltStreamResult::Data(data) => Ok(data),
            AltStreamResult::Error(err) => Err(Box::new(std::io::Error::other(err))),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ChannelEmotes {
    pub channel: String,
    pub emotes: Vec<ChannelEmoteInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ChannelEmoteInfo {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub display_order: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ChannelRewards {
    pub channel: String,
    pub rewards: Vec<ChannelRewardInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct Panels {
    pub channel: String,
    pub panels: Vec<PanelInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct PanelInfo {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub content: String, // TODO: this can seemingly be other types as well
    pub display_order: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct ChannelInfo {
    pub avatar_url: String,
    pub bio: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    pub display_name: String,
    pub stream: StreamInfo,
    pub username: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct StreamTags {
    pub channel: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
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
