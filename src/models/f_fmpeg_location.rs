/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// FFmpegLocation : Enum describing the location of the FFmpeg tool.

/// Enum describing the location of the FFmpeg tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FFmpegLocation {
    #[serde(rename = "NotFound")]
    NotFound,
    #[serde(rename = "SetByArgument")]
    SetByArgument,
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "System")]
    System,
}

impl ToString for FFmpegLocation {
    fn to_string(&self) -> String {
        match self {
            Self::NotFound => String::from("NotFound"),
            Self::SetByArgument => String::from("SetByArgument"),
            Self::Custom => String::from("Custom"),
            Self::System => String::from("System"),
        }
    }
}

impl Default for FFmpegLocation {
    fn default() -> FFmpegLocation {
        Self::NotFound
    }
}
