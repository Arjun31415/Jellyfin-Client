/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// PlayCommand : Enum PlayCommand.

/// Enum PlayCommand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayCommand {
    #[serde(rename = "PlayNow")]
    PlayNow,
    #[serde(rename = "PlayNext")]
    PlayNext,
    #[serde(rename = "PlayLast")]
    PlayLast,
    #[serde(rename = "PlayInstantMix")]
    PlayInstantMix,
    #[serde(rename = "PlayShuffle")]
    PlayShuffle,
}

impl ToString for PlayCommand {
    fn to_string(&self) -> String {
        match self {
            Self::PlayNow => String::from("PlayNow"),
            Self::PlayNext => String::from("PlayNext"),
            Self::PlayLast => String::from("PlayLast"),
            Self::PlayInstantMix => String::from("PlayInstantMix"),
            Self::PlayShuffle => String::from("PlayShuffle"),
        }
    }
}

impl Default for PlayCommand {
    fn default() -> PlayCommand {
        Self::PlayNow
    }
}
