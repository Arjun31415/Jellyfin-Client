/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// GroupStateType : Enum GroupState.

/// Enum GroupState.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupStateType {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "Playing")]
    Playing,
}

impl ToString for GroupStateType {
    fn to_string(&self) -> String {
        match self {
            Self::Idle => String::from("Idle"),
            Self::Waiting => String::from("Waiting"),
            Self::Paused => String::from("Paused"),
            Self::Playing => String::from("Playing"),
        }
    }
}

impl Default for GroupStateType {
    fn default() -> GroupStateType {
        Self::Idle
    }
}
