use std::{error::Error, format};

use serde::Deserialize;
use time::OffsetDateTime;

const BASE: &str = "https://altstream.online/api";
const VERSION: &str = "v1";

async fn request<T>(url: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let body = reqwest::get(url).await?.text().await?;

    parse_response(&body)
}

fn parse_response<T>(json_str: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let parsed: Response<T> = match serde_json::from_str(json_str) {
        Ok(data) => data,
        Err(err) => {
            return Err(Box::new(std::io::Error::other(format!(
                "Failed to parse JSON: {}\nOriginal JSON: {}",
                err, json_str
            ))));
        }
    };

    parsed.data()
}

pub async fn get_channel_info(username: &str) -> Result<ChannelInfo, Box<dyn Error>> {
    request(&format!("{BASE}/{VERSION}/channels/{username}")).await
}

pub async fn get_channel_panels(username: &str) -> Result<Panels, Box<dyn Error>> {
    request(&format!("{BASE}/{VERSION}/channels/{username}/panels")).await
}

pub async fn get_channel_rewards(username: &str) -> Result<ChannelRewards, Box<dyn Error>> {
    request(&format!("{BASE}/{VERSION}/channels/{username}/rewards")).await
}

pub async fn get_channel_emotes(username: &str) -> Result<ChannelEmotes, Box<dyn Error>> {
    request(&format!("{BASE}/{VERSION}/channels/{username}/emotes")).await
}

pub async fn get_user_settings(username: &str) -> Result<UserSettings, Box<dyn Error>> {
    let body = reqwest::get(&format!("{BASE}/user-settings?username={username}"))
        .await?
        .text()
        .await?;

    match serde_json::from_str(&body) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(std::io::Error::other(format!(
            "Failed to parse JSON: {}\nOriginal JSON: {}",
            err, body
        )))),
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
enum Response<T> {
    Data(T),
    Error(String),
}

impl<T> Response<T> {
    fn data(self) -> Result<T, Box<dyn Error>> {
        match self {
            Response::Data(data) => Ok(data),
            Response::Error(err) => Err(Box::new(std::io::Error::other(err))),
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
    pub header_image: String,
    pub offline_image: String,
    pub bio: String,
    pub pronouns: String,
    pub bluesky_url: String,
    pub website_url: String,
    pub discord_url: String,
    pub kofi_url: String,
    pub chat_color: String,
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
    pub description: String,
    pub cost: u64,
    pub enabled: bool,
    pub is_highlight: bool,
    pub cooldown_seconds: Option<u64>,
    pub global_cooldown_seconds: Option<u64>,
    pub max_per_stream: Option<u64>,
    pub global_max_per_stream: Option<u64>,
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
pub struct GameInfo {
    pub id: i64,
    pub igdb_id: Option<i64>,
    pub name: String,
    pub summary: Option<String>,
    pub slug: String,
    pub cover_url: String,
}
