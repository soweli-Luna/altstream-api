//! # altstream-api
//!
//! Rust API wrapper for the alternative streaming platform [AltStream Online](https://altstream.online/)
//!
//! ## Disclaimer
//!
//! This library is not well tested, and the AltStream API is brand new and may see breaking changes in the future. Use at your own risk.
//!
//! ## Example
//!
//! ```
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let username = "letsgoashe"; // The creator of AltStream!
//! let channel_info = altstream_api::get_channel_info(username).await?;
//!
//! let bio: Option<String> = channel_info.bio;
//!
//! let live: bool = channel_info.stream.is_live;
//!
//! let game: String = channel_info.stream.game.ok_or("No game")?.name;
//!
//! # Ok(())
//! # }
//! ```

mod api;
mod request;
mod transmission_types;

pub use api::*;
pub use transmission_types::*;
